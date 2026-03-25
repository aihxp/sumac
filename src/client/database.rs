use postgres::{Client, NoTls};
use rusqlite::Connection;
use serde_json::{json, Value};

use crate::error::{Result, SxmcError};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DatabaseType {
    Sqlite,
    Postgres,
}

struct DatabaseEntryParts<'a> {
    schema: Option<&'a str>,
    name: &'a str,
    object_type: &'a str,
    sql: Option<Value>,
    columns: Vec<Value>,
    foreign_keys: Vec<Value>,
    indexes: Vec<Value>,
}

pub fn inspect_database(
    source: &str,
    database_type: Option<&str>,
    table: Option<&str>,
    search: Option<&str>,
    compact: bool,
) -> Result<Value> {
    match detect_database_type(source, database_type)? {
        DatabaseType::Sqlite => inspect_sqlite(source, table, search, compact),
        DatabaseType::Postgres => inspect_postgres(source, table, search, compact),
    }
}

fn detect_database_type(source: &str, database_type: Option<&str>) -> Result<DatabaseType> {
    if let Some(database_type) = database_type {
        return match database_type {
            "sqlite" => Ok(DatabaseType::Sqlite),
            "postgres" => Ok(DatabaseType::Postgres),
            other => Err(SxmcError::Other(format!(
                "Unsupported database type '{}'. Use sqlite or postgres.",
                other
            ))),
        };
    }

    let lower = source.to_ascii_lowercase();
    if lower.starts_with("postgres://") || lower.starts_with("postgresql://") {
        return Ok(DatabaseType::Postgres);
    }
    if lower.ends_with(".sqlite") || lower.ends_with(".db") || lower.ends_with(".sqlite3") {
        return Ok(DatabaseType::Sqlite);
    }
    if std::path::Path::new(source).exists() {
        return Ok(DatabaseType::Sqlite);
    }

    Err(SxmcError::Other(format!(
        "Could not determine database type for '{}'. Use --database-type sqlite|postgres.",
        source
    )))
}

fn inspect_sqlite(
    source: &str,
    table: Option<&str>,
    search: Option<&str>,
    compact: bool,
) -> Result<Value> {
    let conn = Connection::open(source).map_err(|e| {
        SxmcError::Other(format!(
            "Failed to open SQLite database '{}': {}",
            source, e
        ))
    })?;

    let mut stmt = conn
        .prepare(
            "SELECT name, type, COALESCE(sql, '') \
             FROM sqlite_schema \
             WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%' \
             ORDER BY name",
        )
        .map_err(|e| SxmcError::Other(format!("Failed to inspect SQLite schema: {}", e)))?;

    let search_lower = search.map(|value| value.to_ascii_lowercase());
    let explicit_table = table.map(|value| value.to_string());
    let rows = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let object_type: String = row.get(1)?;
            let sql: String = row.get(2)?;
            Ok((name, object_type, sql))
        })
        .map_err(|e| SxmcError::Other(format!("Failed to query SQLite schema: {}", e)))?;

    let mut entries = Vec::new();
    for row in rows {
        let (name, object_type, sql) =
            row.map_err(|e| SxmcError::Other(format!("Failed to read SQLite schema row: {}", e)))?;

        if let Some(expected) = explicit_table.as_deref() {
            if name != expected {
                continue;
            }
        }

        if let Some(pattern) = search_lower.as_deref() {
            let haystack = format!("{} {} {}", name, object_type, sql).to_ascii_lowercase();
            if !haystack.contains(pattern) {
                continue;
            }
        }

        let columns = inspect_sqlite_columns(&conn, &name, &object_type)?;
        let foreign_keys = inspect_sqlite_foreign_keys(&conn, &name, &object_type)?;
        let indexes = inspect_sqlite_indexes(&conn, &name, &object_type)?;
        entries.push(database_entry_value(
            DatabaseEntryParts {
                schema: None,
                name: &name,
                object_type: &object_type,
                sql: if sql.is_empty() {
                    None
                } else {
                    Some(Value::String(sql))
                },
                columns,
                foreign_keys,
                indexes,
            },
            compact,
        ));
    }

    if let Some(expected) = explicit_table {
        if entries.is_empty() {
            return Err(SxmcError::Other(format!(
                "Table or view '{}' was not found in SQLite database '{}'.",
                expected, source
            )));
        }
    }

    Ok(json!({
        "discovery_schema": "sxmc_discover_db_v1",
        "source_type": "database",
        "database_type": "sqlite",
        "source": source,
        "selected_table": table,
        "search": search,
        "compact": compact,
        "count": entries.len(),
        "entries": entries,
    }))
}

fn inspect_postgres(
    source: &str,
    table: Option<&str>,
    search: Option<&str>,
    compact: bool,
) -> Result<Value> {
    let mut client = Client::connect(source, NoTls).map_err(|e| {
        SxmcError::Other(format!(
            "Failed to connect to PostgreSQL database '{}': {}",
            source, e
        ))
    })?;

    let search_lower = search.map(|value| value.to_ascii_lowercase());
    let explicit_table = table.map(|value| value.to_string());
    let rows = client
        .query(
            "SELECT table_schema, table_name, table_type \
             FROM information_schema.tables \
             WHERE table_schema NOT IN ('pg_catalog', 'information_schema') \
             ORDER BY table_schema, table_name",
            &[],
        )
        .map_err(|e| SxmcError::Other(format!("Failed to inspect PostgreSQL schema: {}", e)))?;

    let mut entries = Vec::new();
    for row in rows {
        let schema: String = row.get(0);
        let name: String = row.get(1);
        let table_type: String = row.get(2);
        let object_type = if table_type.eq_ignore_ascii_case("VIEW") {
            "view"
        } else {
            "table"
        };
        let qualified_name = format!("{}.{}", schema, name);

        if let Some(expected) = explicit_table.as_deref() {
            if name != expected && qualified_name != expected {
                continue;
            }
        }

        if let Some(pattern) = search_lower.as_deref() {
            let haystack = format!("{} {} {}", schema, name, object_type).to_ascii_lowercase();
            if !haystack.contains(pattern) {
                continue;
            }
        }

        let columns = inspect_postgres_columns(&mut client, &schema, &name)?;
        let foreign_keys = inspect_postgres_foreign_keys(&mut client, &schema, &name)?;
        let indexes = inspect_postgres_indexes(&mut client, &schema, &name)?;
        entries.push(database_entry_value(
            DatabaseEntryParts {
                schema: Some(&schema),
                name: &name,
                object_type,
                sql: None,
                columns,
                foreign_keys,
                indexes,
            },
            compact,
        ));
    }

    if let Some(expected) = explicit_table {
        if entries.is_empty() {
            return Err(SxmcError::Other(format!(
                "Table or view '{}' was not found in PostgreSQL database '{}'.",
                expected, source
            )));
        }
    }

    Ok(json!({
        "discovery_schema": "sxmc_discover_db_v1",
        "source_type": "database",
        "database_type": "postgres",
        "source": source,
        "selected_table": table,
        "search": search,
        "compact": compact,
        "count": entries.len(),
        "entries": entries,
    }))
}

fn database_entry_value(entry: DatabaseEntryParts<'_>, compact: bool) -> Value {
    let qualified_name = entry
        .schema
        .map(|schema| format!("{}.{}", schema, entry.name))
        .unwrap_or_else(|| entry.name.to_string());
    let mut value = json!({
        "schema": entry.schema,
        "name": entry.name,
        "qualified_name": qualified_name,
        "object_type": entry.object_type,
        "sql": entry.sql.unwrap_or(Value::Null),
        "column_count": entry.columns.len(),
        "foreign_key_count": entry.foreign_keys.len(),
        "index_count": entry.indexes.len(),
    });

    if !compact {
        value["columns"] = Value::Array(entry.columns);
        value["foreign_keys"] = Value::Array(entry.foreign_keys);
        value["indexes"] = Value::Array(entry.indexes);
    }

    value
}

fn inspect_sqlite_columns(
    conn: &Connection,
    table_name: &str,
    object_type: &str,
) -> Result<Vec<Value>> {
    if object_type != "table" {
        return Ok(Vec::new());
    }

    let pragma = format!(
        "PRAGMA table_info({})",
        sqlite_identifier_literal(table_name)
    );
    let mut stmt = conn.prepare(&pragma).map_err(|e| {
        SxmcError::Other(format!(
            "Failed to inspect columns for '{}': {}",
            table_name, e
        ))
    })?;

    let rows = stmt
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let data_type: String = row.get(2)?;
            let not_null: i64 = row.get(3)?;
            let default_value: Option<String> = row.get(4)?;
            let primary_key_position: i64 = row.get(5)?;
            Ok(json!({
                "name": name,
                "data_type": data_type,
                "not_null": not_null != 0,
                "default": default_value,
                "primary_key": primary_key_position != 0,
                "primary_key_position": primary_key_position,
            }))
        })
        .map_err(|e| {
            SxmcError::Other(format!(
                "Failed to read columns for '{}': {}",
                table_name, e
            ))
        })?;

    collect_rows(rows, table_name, "column")
}

fn inspect_sqlite_foreign_keys(
    conn: &Connection,
    table_name: &str,
    object_type: &str,
) -> Result<Vec<Value>> {
    if object_type != "table" {
        return Ok(Vec::new());
    }

    let pragma = format!(
        "PRAGMA foreign_key_list({})",
        sqlite_identifier_literal(table_name)
    );
    let mut stmt = conn.prepare(&pragma).map_err(|e| {
        SxmcError::Other(format!(
            "Failed to inspect foreign keys for '{}': {}",
            table_name, e
        ))
    })?;

    let rows = stmt
        .query_map([], |row| {
            let id: i64 = row.get(0)?;
            let seq: i64 = row.get(1)?;
            let ref_table: String = row.get(2)?;
            let from: String = row.get(3)?;
            let to: String = row.get(4)?;
            let on_update: String = row.get(5)?;
            let on_delete: String = row.get(6)?;
            Ok(json!({
                "id": id,
                "sequence": seq,
                "column": from,
                "references_table": ref_table,
                "references_column": to,
                "on_update": on_update,
                "on_delete": on_delete,
            }))
        })
        .map_err(|e| {
            SxmcError::Other(format!(
                "Failed to read foreign keys for '{}': {}",
                table_name, e
            ))
        })?;

    collect_rows(rows, table_name, "foreign key")
}

fn inspect_sqlite_indexes(
    conn: &Connection,
    table_name: &str,
    object_type: &str,
) -> Result<Vec<Value>> {
    if object_type != "table" {
        return Ok(Vec::new());
    }

    let pragma = format!(
        "PRAGMA index_list({})",
        sqlite_identifier_literal(table_name)
    );
    let mut stmt = conn.prepare(&pragma).map_err(|e| {
        SxmcError::Other(format!(
            "Failed to inspect indexes for '{}': {}",
            table_name, e
        ))
    })?;

    let rows = stmt
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let unique: i64 = row.get(2)?;
            let origin: String = row.get(3)?;
            let partial: i64 = row.get(4)?;
            Ok(json!({
                "name": name,
                "unique": unique != 0,
                "origin": origin,
                "partial": partial != 0,
            }))
        })
        .map_err(|e| {
            SxmcError::Other(format!(
                "Failed to read indexes for '{}': {}",
                table_name, e
            ))
        })?;

    collect_rows(rows, table_name, "index")
}

fn inspect_postgres_columns(
    client: &mut Client,
    schema: &str,
    table_name: &str,
) -> Result<Vec<Value>> {
    let rows = client
        .query(
            "SELECT column_name, udt_name, is_nullable, column_default \
             FROM information_schema.columns \
             WHERE table_schema = $1 AND table_name = $2 \
             ORDER BY ordinal_position",
            &[&schema, &table_name],
        )
        .map_err(|e| {
            SxmcError::Other(format!(
                "Failed to inspect PostgreSQL columns for '{}.{}': {}",
                schema, table_name, e
            ))
        })?;

    Ok(rows
        .into_iter()
        .map(|row| {
            json!({
                "name": row.get::<_, String>(0),
                "data_type": row.get::<_, String>(1),
                "not_null": row.get::<_, String>(2).eq_ignore_ascii_case("NO"),
                "default": row.get::<_, Option<String>>(3),
            })
        })
        .collect())
}

fn inspect_postgres_foreign_keys(
    client: &mut Client,
    schema: &str,
    table_name: &str,
) -> Result<Vec<Value>> {
    let rows = client
        .query(
            "SELECT tc.constraint_name, kcu.column_name, ccu.table_schema, ccu.table_name, ccu.column_name \
             FROM information_schema.table_constraints tc \
             JOIN information_schema.key_column_usage kcu \
               ON tc.constraint_name = kcu.constraint_name AND tc.table_schema = kcu.table_schema \
             JOIN information_schema.constraint_column_usage ccu \
               ON tc.constraint_name = ccu.constraint_name AND tc.table_schema = ccu.table_schema \
             WHERE tc.constraint_type = 'FOREIGN KEY' AND tc.table_schema = $1 AND tc.table_name = $2 \
             ORDER BY tc.constraint_name, kcu.ordinal_position",
            &[&schema, &table_name],
        )
        .map_err(|e| {
            SxmcError::Other(format!(
                "Failed to inspect PostgreSQL foreign keys for '{}.{}': {}",
                schema, table_name, e
            ))
        })?;

    Ok(rows
        .into_iter()
        .map(|row| {
            json!({
                "name": row.get::<_, String>(0),
                "column": row.get::<_, String>(1),
                "references_schema": row.get::<_, String>(2),
                "references_table": row.get::<_, String>(3),
                "references_column": row.get::<_, String>(4),
            })
        })
        .collect())
}

fn inspect_postgres_indexes(
    client: &mut Client,
    schema: &str,
    table_name: &str,
) -> Result<Vec<Value>> {
    let rows = client
        .query(
            "SELECT indexname, indexdef \
             FROM pg_indexes \
             WHERE schemaname = $1 AND tablename = $2 \
             ORDER BY indexname",
            &[&schema, &table_name],
        )
        .map_err(|e| {
            SxmcError::Other(format!(
                "Failed to inspect PostgreSQL indexes for '{}.{}': {}",
                schema, table_name, e
            ))
        })?;

    Ok(rows
        .into_iter()
        .map(|row| {
            json!({
                "name": row.get::<_, String>(0),
                "definition": row.get::<_, String>(1),
            })
        })
        .collect())
}

fn collect_rows<T>(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>>,
    table_name: &str,
    row_type: &str,
) -> Result<Vec<T>> {
    let mut values = Vec::new();
    for row in rows {
        values.push(row.map_err(|e| {
            SxmcError::Other(format!(
                "Failed to decode SQLite {} for '{}': {}",
                row_type, table_name, e
            ))
        })?);
    }
    Ok(values)
}

fn sqlite_identifier_literal(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

#[cfg(test)]
mod tests {
    use super::{detect_database_type, DatabaseType};

    #[test]
    fn detects_postgres_urls() {
        assert_eq!(
            detect_database_type("postgres://demo:demo@localhost/test", None).unwrap(),
            DatabaseType::Postgres
        );
        assert_eq!(
            detect_database_type("postgresql://demo:demo@localhost/test", None).unwrap(),
            DatabaseType::Postgres
        );
    }

    #[test]
    fn detects_sqlite_paths() {
        assert_eq!(
            detect_database_type("/tmp/demo.sqlite", None).unwrap(),
            DatabaseType::Sqlite
        );
        assert_eq!(
            detect_database_type("/tmp/demo.db", None).unwrap(),
            DatabaseType::Sqlite
        );
    }
}

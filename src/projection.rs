use serde_json::Value;

pub fn apply_offset_limit<T>(items: &mut Vec<T>, offset: Option<usize>, limit: Option<usize>) {
    if let Some(offset) = offset {
        if offset >= items.len() {
            items.clear();
        } else if offset > 0 {
            items.drain(0..offset);
        }
    }
    if let Some(limit) = limit {
        items.truncate(limit);
    }
}

pub fn retain_object_fields(value: Value, fields: &[String]) -> Value {
    let Some(object) = value.as_object() else {
        return value;
    };
    let mut filtered = serde_json::Map::new();
    for field in fields {
        if let Some(item) = object.get(field) {
            filtered.insert(field.clone(), item.clone());
        }
    }
    Value::Object(filtered)
}

use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;
use std::time::Duration;

use sxmc::cli_surfaces::AiClientProfile;
use sxmc::error::Result;
use sxmc::output;
use sxmc::paths::InstallPaths;

use crate::cli_args::WatchNotificationTemplate;
use crate::{render_status_output, status_has_unhealthy_baked_health, status_value_with_health};

use super::CommandOutcome;

#[derive(Clone)]
pub(crate) struct WatchNotificationOptions {
    pub(crate) file: Option<PathBuf>,
    pub(crate) command: Option<String>,
    pub(crate) webhooks: Vec<String>,
    pub(crate) slack_webhooks: Vec<String>,
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) template: WatchNotificationTemplate,
}

#[derive(Clone)]
pub(crate) struct WatchRequest {
    pub(crate) install_paths: InstallPaths,
    pub(crate) only_hosts: Vec<AiClientProfile>,
    pub(crate) compare_hosts: Vec<AiClientProfile>,
    pub(crate) health: bool,
    pub(crate) interval: Duration,
    pub(crate) exit_on_change: bool,
    pub(crate) exit_on_unhealthy: bool,
    pub(crate) notify: WatchNotificationOptions,
    pub(crate) pretty: bool,
    pub(crate) format: Option<output::StructuredOutputFormat>,
}

pub(crate) struct WatchService;

impl WatchService {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn run(&self, request: WatchRequest) -> Result<CommandOutcome> {
        let stdout_is_tty = std::io::stdout().is_terminal();
        let mut last_rendered = None::<String>;
        let mut first_frame = true;

        loop {
            let value = status_value_with_health(
                &request.install_paths,
                &request.only_hosts,
                &request.compare_hosts,
                request.health,
            )
            .await?;
            let rendered =
                render_status_output(&value, request.format, request.pretty, stdout_is_tty);

            if last_rendered.as_ref() != Some(&rendered) {
                println!("{rendered}");
                println!();
                std::io::stdout().flush()?;

                let unhealthy = status_has_unhealthy_baked_health(&value);
                let should_notify = !first_frame || unhealthy;
                if should_notify {
                    let reason = if unhealthy { "unhealthy" } else { "change" };
                    let event = watch_event_value(&request.install_paths, reason, &value);
                    let payload = watch_notification_payload(request.notify.template, &event);
                    if let Some(path) = request.notify.file.as_ref() {
                        append_watch_notification(path, &payload)?;
                    }
                    if let Some(command) = request.notify.command.as_deref() {
                        run_watch_notify_command(
                            command,
                            &event,
                            &payload,
                            request.notify.template,
                        )?;
                    }
                    for webhook in &request.notify.webhooks {
                        send_watch_webhook(webhook, &request.notify.headers, &payload).await?;
                    }
                    if !request.notify.slack_webhooks.is_empty() {
                        let slack_payload =
                            watch_notification_payload(WatchNotificationTemplate::Slack, &event);
                        for webhook in &request.notify.slack_webhooks {
                            send_watch_webhook(webhook, &request.notify.headers, &slack_payload)
                                .await?;
                        }
                    }
                }

                if request.exit_on_unhealthy && unhealthy {
                    return Ok(CommandOutcome { exit_code: Some(1) });
                }
                if request.exit_on_change && !first_frame {
                    return Ok(CommandOutcome { exit_code: Some(1) });
                }

                last_rendered = Some(rendered);
            }

            first_frame = false;
            tokio::time::sleep(request.interval).await;
        }
    }
}

fn watch_event_value(install_paths: &InstallPaths, reason: &str, value: &Value) -> Value {
    json!({
        "event_schema": "sxmc_watch_event_v1",
        "reason": reason,
        "root": install_paths.project_root().display().to_string(),
        "install_scope": install_paths.scope().as_str(),
        "observed_at": Utc::now().to_rfc3339(),
        "status": value,
    })
}

fn watch_notification_payload(template: WatchNotificationTemplate, event: &Value) -> Value {
    let status = &event["status"];
    let sync_state = &status["sync_state"];
    let recovery_plan = &status["recovery_plan"];
    let ai_hosts = status["ai_knowledge"]["hosts"]
        .as_object()
        .map(|hosts| hosts.len())
        .unwrap_or(0);

    let compact = json!({
        "event_schema": "sxmc_watch_notification_v1",
        "template": match template {
            WatchNotificationTemplate::Standard => "standard",
            WatchNotificationTemplate::Compact => "compact",
            WatchNotificationTemplate::Slack => "slack",
        },
        "reason": event["reason"],
        "root": event["root"],
        "observed_at": event["observed_at"],
        "summary": {
            "host_count": ai_hosts,
            "drift_count": sync_state["current_drift_count"].as_u64().unwrap_or(0),
            "recovery_count": recovery_plan["count"].as_u64().unwrap_or(0),
            "unhealthy_baked_count": status["baked_health"]["unhealthy_count"].as_u64().unwrap_or(0),
        },
        "commands_needing_sync": sync_state["commands_needing_sync"].clone(),
        "top_recovery_items": recovery_plan["items"]
            .as_array()
            .map(|items| items.iter().take(3).cloned().collect::<Vec<_>>())
            .unwrap_or_default(),
    });

    match template {
        WatchNotificationTemplate::Standard => event.clone(),
        WatchNotificationTemplate::Compact => compact,
        WatchNotificationTemplate::Slack => {
            let drift_count = compact["summary"]["drift_count"].as_u64().unwrap_or(0);
            let recovery_count = compact["summary"]["recovery_count"].as_u64().unwrap_or(0);
            let unhealthy_count = compact["summary"]["unhealthy_baked_count"]
                .as_u64()
                .unwrap_or(0);
            let root = event["root"].as_str().unwrap_or(".");
            let reason = event["reason"].as_str().unwrap_or("change");
            let text = format!(
                "sxmc watch {reason} for {root} — drift: {drift_count}, recovery: {recovery_count}, unhealthy: {unhealthy_count}"
            );
            json!({
                "text": text,
                "blocks": [
                    {
                        "type": "section",
                        "text": {
                            "type": "mrkdwn",
                            "text": format!("*sxmc watch {}*\n`{}`", reason, root),
                        }
                    },
                    {
                        "type": "section",
                        "fields": [
                            { "type": "mrkdwn", "text": format!("*Drift*\n{}", drift_count) },
                            { "type": "mrkdwn", "text": format!("*Recovery*\n{}", recovery_count) },
                            { "type": "mrkdwn", "text": format!("*Unhealthy*\n{}", unhealthy_count) },
                            { "type": "mrkdwn", "text": format!("*Observed*\n{}", event["observed_at"].as_str().unwrap_or("")) }
                        ]
                    }
                ],
                "sxmc_event": compact,
            })
        }
    }
}

fn append_watch_notification(path: &Path, payload: &Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(file, "{}", serde_json::to_string(payload)?)?;
    Ok(())
}

fn run_watch_notify_command(
    command: &str,
    event: &Value,
    payload: &Value,
    template: WatchNotificationTemplate,
) -> Result<()> {
    let temp_event_path = std::env::temp_dir().join(format!(
        "sxmc-watch-event-{}-{}.json",
        std::process::id(),
        Utc::now().timestamp_micros()
    ));
    let temp_payload_path = std::env::temp_dir().join(format!(
        "sxmc-watch-payload-{}-{}.json",
        std::process::id(),
        Utc::now().timestamp_micros()
    ));
    fs::write(&temp_event_path, serde_json::to_string_pretty(event)?)?;
    fs::write(&temp_payload_path, serde_json::to_string_pretty(payload)?)?;

    let mut child = if cfg!(windows) {
        let mut cmd = StdCommand::new("cmd");
        cmd.arg("/C").arg(command);
        cmd
    } else {
        let mut cmd = StdCommand::new("sh");
        cmd.arg("-lc").arg(command);
        cmd
    };

    child
        .env(
            "SXMC_WATCH_REASON",
            event["reason"].as_str().unwrap_or("change"),
        )
        .env("SXMC_WATCH_EVENT_PATH", temp_event_path.as_os_str())
        .env("SXMC_WATCH_PAYLOAD_PATH", temp_payload_path.as_os_str())
        .env("SXMC_WATCH_ROOT", event["root"].as_str().unwrap_or("."));
    child.env(
        "SXMC_WATCH_NOTIFY_TEMPLATE",
        match template {
            WatchNotificationTemplate::Standard => "standard",
            WatchNotificationTemplate::Compact => "compact",
            WatchNotificationTemplate::Slack => "slack",
        },
    );

    let status = child.status()?;
    if !status.success() {
        eprintln!("[sxmc] Watch notify command exited with status {}", status);
    }
    let _ = fs::remove_file(temp_event_path);
    let _ = fs::remove_file(temp_payload_path);
    Ok(())
}

async fn send_watch_webhook(url: &str, headers: &[(String, String)], event: &Value) -> Result<()> {
    let mut request = reqwest::Client::new().post(url).json(event);
    for (key, value) in headers {
        request = request.header(key, value);
    }
    let response = request.send().await.map_err(|error| {
        sxmc::error::SxmcError::Other(format!(
            "Failed to POST watch notification to '{}': {}",
            url, error
        ))
    })?;
    if !response.status().is_success() {
        return Err(sxmc::error::SxmcError::Other(format!(
            "Watch webhook '{}' returned HTTP {}",
            url,
            response.status()
        )));
    }
    Ok(())
}

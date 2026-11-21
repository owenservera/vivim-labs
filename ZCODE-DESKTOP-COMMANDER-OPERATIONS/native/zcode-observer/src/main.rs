//! Read-only ZCode native NDJSON and V4 snapshot observer.
//! Owns a fresh app-server child. Never modifies existing sessions or providers.
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdin, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const DEFAULT_NODE: &str = r"C:\Program Files\nodejs\node.exe";
const DEFAULT_ZCODE: &str = r"C:\Program Files\ZCode\resources\glm\zcode.cjs";
const DEFAULT_WORKSPACE: &str = r"C:\0-BlackBoxProject-0\ARBOR";
const MAX_FRAME_BYTES: usize = 2 * 1024 * 1024;

#[derive(Default)]
struct Checks {
    sessions: bool,
    workflows: bool,
    plugins: bool,
    command_query: bool,
    subscribe: bool,
    snapshot: bool,
    parent_projection: bool,
    notifications: usize,
    frames: usize,
    errors: usize,
}

fn send(stdin: &mut ChildStdin, id: &str, method: &str, params: Value) -> Result<(), String> {
    let request = json!({"id": id, "method": method, "params": params});
    serde_json::to_writer(&mut *stdin, &request).map_err(|e| e.to_string())?;
    stdin.write_all(b"\n").map_err(|e| e.to_string())?;
    stdin.flush().map_err(|e| e.to_string())
}

fn workspace_id(reply: &Value) -> Option<&str> {
    let workspace = reply.pointer("/result/sessions/0/workspace")?;
    ["workspaceIdentity", "workspaceKey", "workspacePath"]
        .iter()
        .find_map(|key| {
            workspace
                .get(*key)
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
        })
}

fn count(reply: &Value, field: &str) -> Option<usize> {
    reply
        .pointer(&format!("/result/{field}"))?
        .as_array()
        .map(Vec::len)
}

fn inspect_frame_cursor(frame: &Value) -> (bool, bool) {
    // Native V4 uses a physical complete wire containing a logical frame.
    // The logical frame carries toSeq; its snapshot payload holds logEpoch.
    let logical = frame.pointer("/params/frame").unwrap_or(&Value::Null);
    let has_epoch = logical
        .pointer("/payload/snapshot/logEpoch")
        .and_then(Value::as_str)
        .is_some();
    let has_sequence = logical.get("toSeq").and_then(Value::as_u64).is_some();
    (has_epoch, has_sequence)
}

fn run() -> Result<(), String> {
    let node = std::env::var("ZCODE_OBSERVER_NODE").unwrap_or_else(|_| DEFAULT_NODE.into());
    let entry = std::env::var("ZCODE_OBSERVER_ENTRY").unwrap_or_else(|_| DEFAULT_ZCODE.into());
    let repo =
        std::env::var("ZCODE_OBSERVER_WORKSPACE").unwrap_or_else(|_| DEFAULT_WORKSPACE.into());

    let mut child = Command::new(&node)
        .arg(&entry)
        .arg("app-server")
        .current_dir(&repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("APP_SERVER_START_FAIL {e}"))?;
    let mut stdin = child.stdin.take().ok_or("MISSING_STDIN")?;
    let stdout = child.stdout.take().ok_or("MISSING_STDOUT")?;
    let mut stderr = child.stderr.take().ok_or("MISSING_STDERR")?;
    let (tx, rx) = mpsc::channel::<Result<Value, String>>();
    let reader = thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let result = match line {
                Ok(line) if line.len() > MAX_FRAME_BYTES => Err("FRAME_TOO_LARGE".into()),
                Ok(line) => serde_json::from_str(&line).map_err(|_| "BAD_NDJSON".into()),
                Err(_) => Err("READ_ERROR".into()),
            };
            if tx.send(result).is_err() {
                break;
            }
        }
    });
    let stderr_reader = thread::spawn(move || {
        let mut sink = std::io::sink();
        let _ = std::io::copy(&mut stderr, &mut sink);
    });

    let result = (|| {
        // The installed server sends initial storage notifications during startup.
        thread::sleep(Duration::from_millis(2200));
        send(&mut stdin, "sessions", "session/list", json!({"limit": 1}))?;
        send(
            &mut stdin,
            "plugins",
            "plugins/list",
            json!({"workspace": {"workspacePath": repo, "workspaceKey": repo}}),
        )?;
        send(
            &mut stdin,
            "workflows",
            "workflows/list",
            json!({"workspace": {"workspacePath": repo, "workspaceKey": repo}}),
        )?;
        // Query an intentionally nonexistent global command: never issue a command.
        send(
            &mut stdin,
            "query",
            "v4/commands/query",
            json!({"commands":[{"sessionId": null,"commandId": "r003-rust-observer-read-only"}],"clock":true}),
        )?;

        let parent_probe = std::env::var("ZCODE_OBSERVER_PARENT_SESSION")
            .ok()
            .filter(|s| !s.is_empty() && s.len() <= 256);
        if let Some(ref parent_session_id) = parent_probe {
            send(
                &mut stdin,
                "parent",
                "session/subagents",
                json!({"sessionId": parent_session_id, "endedLimit": 20}),
            )?;
        }
        let mut checks = Checks::default();
        let deadline = Instant::now() + Duration::from_secs(38);
        while Instant::now() < deadline {
            let frame = match rx.recv_timeout(Duration::from_millis(350)) {
                Ok(Ok(frame)) => frame,
                Ok(Err(reason)) => {
                    eprintln!("RUST_FRAME_ERROR={reason}");
                    checks.errors += 1;
                    continue;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            };
            if frame["method"] == "v4/conversation/frame" {
                checks.frames += 1;
                let (epoch, seq) = inspect_frame_cursor(&frame);
                let complete = frame.pointer("/params/kind") == Some(&json!("complete"));
                let snapshot =
                    frame.pointer("/params/frame/payload/kind") == Some(&json!("snapshot"));
                let owned = frame.pointer("/params/subscriptionId")
                    == frame.pointer("/params/frame/subscriptionId");
                println!("RUST_V4_FRAME={} complete={} snapshot={} ownership={} cursor_epoch_present={} cursor_seq_present={}",
                    checks.frames, complete, snapshot, owned, epoch, seq);
                checks.snapshot |= complete && snapshot && owned && epoch && seq;
            } else if frame.get("method").is_some() {
                checks.notifications += 1;
            } else if let Some(id) = frame.get("id").and_then(Value::as_str) {
                if let Some(err) = frame.get("error") {
                    checks.errors += 1;
                    let code = err.get("code").and_then(Value::as_i64).unwrap_or(-1);
                    eprintln!("RUST_QUERY_{id}=FAIL code={code}");
                    continue;
                }
                match id {
                    "sessions" => {
                        if let Some(n) = count(&frame, "sessions") {
                            checks.sessions = true;
                            println!("RUST_SESSIONS=PASS count={n}");
                        }
                        if let Some(ws) = workspace_id(&frame) {
                            send(
                                &mut stdin,
                                "sub",
                                "v4/conversation/subscribe",
                                json!({
                                    "topic": format!("sessions-index/{ws}"),
                                    "visibility": "background",
                                    "connectionId": format!("rust-observer-{}", std::process::id()),
                                    "clientMode": "desktop-continuous"
                                }),
                            )?;
                        } else {
                            checks.errors += 1;
                            eprintln!("RUST_WORKSPACE_ID=MISSING");
                        }
                    }
                    "plugins" => {
                        if let Some(n) = count(&frame, "plugins") {
                            checks.plugins = true;
                            println!("RUST_PLUGINS=PASS count={n}");
                        }
                    }
                    "workflows" => {
                        if let Some(n) = count(&frame, "workflows") {
                            checks.workflows = true;
                            println!("RUST_WORKFLOWS=PASS count={n}");
                        }
                    }
                    "parent" => {
                        let result = &frame["result"];
                        let children = result["childSessionIds"].as_array().map(Vec::len);
                        let running = result["running"].as_array().map(Vec::len);
                        let ended_total = result["ended"]["total"].as_u64();
                        let ended_page = result["ended"]["items"].as_array().map(Vec::len);
                        checks.parent_projection = result["revision"].as_u64().is_some()
                            && children.is_some()
                            && running.is_some()
                            && ended_total.is_some()
                            && ended_page.is_some();
                        println!(
                            "RUST_PARENT_PROJECTION={} children={} running={} ended_total={} ended_page={}",
                            if checks.parent_projection { "PASS" } else { "PARTIAL" },
                            children.unwrap_or(0),
                            running.unwrap_or(0),
                            ended_total.unwrap_or(0),
                            ended_page.unwrap_or(0)
                        );
                    }
                    "query" => {
                        checks.command_query =
                            frame.pointer("/result/results/0/result") == Some(&json!("unknown"));
                        println!(
                            "RUST_COMMAND_QUERY={} unknown_command={}",
                            if checks.command_query {
                                "PASS"
                            } else {
                                "PARTIAL"
                            },
                            checks.command_query
                        );
                    }
                    "sub" => {
                        let ack = &frame["result"]["ack"];
                        checks.subscribe = ack.is_object() && ack["mode"] == "snapshot";
                        println!(
                            "RUST_V4_SUBSCRIBE={} mode={}",
                            if checks.subscribe { "PASS" } else { "PARTIAL" },
                            ack["mode"].as_str().unwrap_or("unknown")
                        );
                    }
                    _ => checks.errors += 1,
                }
            }
            if checks.sessions
                && checks.plugins
                && checks.workflows
                && checks.command_query
                && checks.subscribe
                && checks.snapshot
                && (parent_probe.is_none() || checks.parent_projection)
            {
                break;
            }
        }
        let ok = checks.sessions
            && checks.plugins
            && checks.workflows
            && checks.command_query
            && checks.subscribe
            && checks.snapshot
            && (parent_probe.is_none() || checks.parent_projection)
            && checks.errors == 0;
        println!(
            "RUST_OBSERVER_FINAL={} frames={} notifications={} errors={}",
            if ok { "PASS" } else { "FAIL" },
            checks.frames,
            checks.notifications,
            checks.errors
        );
        if ok {
            Ok(())
        } else {
            Err("INCOMPLETE_PROTOCOL_GATE".into())
        }
    })();

    // Terminate only this process's privately spawned child; leave all live sessions alone.
    drop(stdin);
    let _ = child.kill();
    let _ = child.wait();
    let _ = reader.join();
    let _ = stderr_reader.join();
    result
}

fn main() {
    if let Err(reason) = run() {
        eprintln!("RUST_OBSERVER_EXIT_FAIL={reason}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn workspace_identity_fallback() {
        let frame = json!({"result":{"sessions":[{"workspace":{"workspaceKey":"repo-key"}}]}});
        assert_eq!(workspace_id(&frame), Some("repo-key"));
    }
    #[test]
    fn command_query_shape() {
        let frame = json!({"result":{"results":[{"key":{"sessionId":null,"commandId":"x"},"result":"unknown"}]}});
        assert_eq!(
            frame.pointer("/result/results/0/result"),
            Some(&json!("unknown"))
        );
    }
    #[test]
    fn cursor_probe_does_not_read_user_content() {
        let frame = json!({"params":{"kind":"complete","frame":{"toSeq":2,"payload":{"kind":"snapshot","snapshot":{"logEpoch":"e","secret":"do-not-print"}}}}});
        assert_eq!(inspect_frame_cursor(&frame), (true, true));
    }
}

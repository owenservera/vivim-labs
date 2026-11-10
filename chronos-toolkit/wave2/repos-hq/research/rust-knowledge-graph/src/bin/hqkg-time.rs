//! Chronos read-only CLI: deterministic calculations and JSON-line results.
//! No timer daemon, Host execution, scheduler registration, or data collection.
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use hqkg::chronos::{ChronosCompute, SystemClock, SLOT, INTERFACE};
use hqkg::{ComputeEngine, EngineRequest, Error, Result};

fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch < '\u{20}' => out.push_str(&format!("\\u{:04x}", ch as u32)),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn help() -> &'static str {
    "hqkg-time now | duration <start_utc_ms> <end_utc_ms> | deadline <due_utc_ms> [grace_ms] | \
     next <anchor_utc_ms> <interval_ms> <from_utc_ms> | window <start_ms> <end_ms> <query_ms> | \
     elapsed <monotonic_ms> <boot_id> | freshness <observed_ms> <ttl_ms> [skew_ms] | heartbeat <seen_ms> <timeout_ms> | retry <base_ms> <cap_ms> <attempt> | due <anchor_ms> <interval_ms> <last_checked_ms> <now_ms> [max_occurrences] | capabilities\n\
     Read-only: no persistence, permissions, automation registration, or external effects.\n\
     Output is a single vivim.chronos.cli-result/1 JSON object per call."
}
fn run(args: &[String]) -> Result<String> {
    let Some(command) = args.first().map(String::as_str) else { return Err(Error::Invalid(help().into())); };
    if command == "capabilities" {
        if args.len() != 1 { return Err(Error::Invalid(help().into())); }
        return Ok(format!("{{\"schema\":\"vivim.chronos.cli-result/1\",\"interface\":{},\"slot\":{},\"operations\":[\"now\",\"duration\",\"deadline\",\"next\",\"window\",\"elapsed\",\"freshness\",\"heartbeat\",\"retry\",\"due\"],\"effectful\":false}}",
            quote(INTERFACE), quote(SLOT)));
    }
    let required = match command {
        "now" => (1, 1), "duration" => (3, 3), "deadline" => (2, 3),
        "next" => (4, 4), "window" => (4, 4), "elapsed" => (3, 3),
        "freshness" => (3, 4), "heartbeat" => (3, 3), "retry" => (4, 4), "due" => (5, 6),
        _ => return Err(Error::Invalid(help().into())),
    };
    if args.len() < required.0 || args.len() > required.1 { return Err(Error::Invalid(help().into())); }
    // One invocation = one fresh in-process monotonic reference. Host must use its
    // own OS-boot/session identity for spans that survive process boundaries.
    let wall = SystemTime::now().duration_since(UNIX_EPOCH)
        .map_err(|_| Error::Invalid("unavailable system clock".into()))?;
    let boot = format!("cli-{}-{}", std::process::id(), wall.as_nanos());
    let clock = Arc::new(SystemClock::new(&boot)?);
    let engine = ChronosCompute::new(clock);
    let data = if command == "elapsed" { format!("{}|{}", args[1], args[2]) }
        else { args[1..].join(",") };
    let value = engine.execute(&EngineRequest { operation: command.into(), data })?.data;
    Ok(format!("{{\"schema\":\"vivim.chronos.cli-result/1\",\"operation\":{},\"value\":{},\"effectful\":false}}",
        quote(command), quote(&value)))
}
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(s) => println!("{s}"),
        Err(e) => { eprintln!("hqkg-time: {e}"); std::process::exit(2); }
    }
}

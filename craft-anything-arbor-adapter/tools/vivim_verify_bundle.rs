//! Copy this standalone example into Craft's crates/evidence/examples/vivim_verify_bundle.rs,
//! then run: cargo run -p craft-anything-evidence --example vivim_verify_bundle -- BUNDLE_PATH
//! Uses Craft's actual serde_json and Evidence::verify(), never JavaScript reserialization.
//! Prints only a source-bound verification receipt; no raw private target content.
use craft_anything_evidence::Evidence;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, env, fs, path::PathBuf};

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let path = PathBuf::from(args.next().ok_or("expected exactly one bundle path")?);
    if args.next().is_some() { return Err("unexpected extra arguments".into()); }
    let metadata = fs::metadata(&path).map_err(|e| format!("cannot stat bundle: {e}"))?;
    if !metadata.is_file() || metadata.len() > 512 * 1024 * 1024 {
        return Err("bundle missing, not a file, or over 512 MiB".into());
    }
    let bytes = fs::read(&path).map_err(|e| format!("cannot read bundle: {e}"))?;
    let bundle: Value = serde_json::from_slice(&bytes).map_err(|e| format!("invalid JSON: {e}"))?;
    if bundle.get("format").and_then(Value::as_str) != Some("craft-anything-evidence-bundle")
        || bundle.get("version").and_then(Value::as_u64) != Some(1) {
        return Err("unsupported Craft evidence bundle format/version".into());
    }
    let records = bundle.get("records").and_then(Value::as_array).ok_or("missing records")?;
    let unknowns = bundle.get("unknowns").and_then(Value::as_array).ok_or("missing unknowns")?;
    if records.len() > 2500 || unknowns.len() > 2500 {
        return Err("record/unknown limit exceeded for VIVIM research bridge".into());
    }
    let mut ids = BTreeSet::new();
    for (index, raw) in records.iter().enumerate() {
        let record: Evidence = serde_json::from_value(raw.clone())
            .map_err(|e| format!("record {index} cannot parse as Craft Evidence: {e}"))?;
        if !record.verify() {
            return Err(format!("Craft native identity mismatch at record {index}"));
        }
        ids.insert(record.id);
    }
    let receipt = json!({
        "schema": "vivim.craft-native-verification/1",
        "bundleSha256": hex::encode(Sha256::digest(&bytes)),
        "verifiedRecordIds": ids.iter().collect::<Vec<_>>(),
        "totalRecords": records.len(),
        "uniqueRecords": ids.len(),
        "unknowns": unknowns.len(),
        "nativeVerifier": "craft-anything-evidence::Evidence::verify",
        "executionAuthorized": false,
    });
    println!("{}", receipt);
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("VIVIM_NATIVE_VERIFY_FAILED: {error}");
        std::process::exit(1);
    }
}

#!/usr/bin/env python3
"""Inspect and reconcile the one ARBOR master, without a binary connector round-trip.

No product edits, no new index, no dependency installation. Run at Labs root.
All merges compare the original base row and current row for every changed field.
"""
import argparse
import collections
import csv
import gzip
import hashlib
import io
import json
import os
import pathlib
import subprocess
import sys
import tempfile

FOLDER = pathlib.Path(__file__).resolve().parent
ROOT = FOLDER.parent
REL = "ARBOR-DEEP-DISPOSITION/ARBOR-MASTER-INTEGRATION-TRACKER.csv.gz"
MASTER = ROOT / REL
INITIAL = "b5c42977eeae7979ed8495b574767ecb16464c26"
SOURCE = "e07e61385a53f30890b4a7f7441fe25d22a79b1c"
PATHSET = "319f9c8a2ad5792614db1f4a3b1542d4020ea4bb8f26c1fcafc2dd3e232b46f5"
LANES = {"A": 1076, "B": 675, "C": 1189, "D": 748, "E": 1053}
IDENTITY = ("ID", "Granularity", "Repository path", "Symbol / occurrence",
            "Parent file ID", "Parallel lane", "Lane batch", "Lane batch item")
csv.field_size_limit(20_000_000)

def insist(ok, message):
    if not ok:
        raise ValueError(message)

def git(*args):
    return subprocess.check_output(["git", "-C", str(ROOT), *args])

def sha(data):
    return hashlib.sha256(data).hexdigest()

def git_blob(data):
    return hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()

def decode(data):
    raw = gzip.decompress(data)
    reader = csv.DictReader(io.StringIO(raw.decode("utf-8-sig"), newline=""))
    fields = reader.fieldnames
    insist(fields is not None and len(fields) == len(set(fields)), "Invalid headers")
    rows = list(reader)
    insist(all(None not in row and None not in row.values() for row in rows), "Malformed row")
    keyed = {row["ID"]: row for row in rows}
    insist(len(rows) == len(keyed) and all(keyed), "Duplicate or empty stable ID")
    return raw, fields, rows, keyed

def baseline():
    return decode(git("show", INITIAL + ":" + REL))

def validate(fields, rows, keyed, initial):
    ib = initial[3]
    insist(set(ib) == set(keyed), "Unexpected inventory delta; explicit reconciliation needed")
    insist(set(initial[1]).issubset(fields), "Original field missing")
    for sid, row in keyed.items():
        insist(all(row[name] == ib[sid][name] for name in IDENTITY), "Identity changed: " + sid)
    files = {r["ID"]: r for r in rows if r["Granularity"].lower() == "file"}
    paths = [r["Repository path"] for r in files.values()]
    insist(len(paths) == len(set(paths)) == 2558, "File coverage changed")
    insist(sha("\n".join(sorted(paths)).encode()) == PATHSET, "Pinned path set changed")
    for row in rows:
        if row["Granularity"].lower() == "symbol":
            parent = files.get(row["Parent file ID"])
            insist(parent and parent["Repository path"] == row["Repository path"] and
                   parent["Parallel lane"] == row["Parallel lane"], "Broken symbol parent: " + row["ID"])
    insist(collections.Counter(r["Parallel lane"] for r in rows) == LANES, "Lane totals changed")
    batches = collections.Counter((r["Parallel lane"], r["Lane batch"]) for r in rows)
    insist(all(v <= 500 for v in batches.values()), "Oversized review batch")
    return batches

def tally(rows, col):
    return dict(collections.Counter(r.get(col, "") or "(blank)" for r in rows))

def audit(data):
    raw, fields, rows, keyed = decode(data)
    initial = baseline()
    batches = validate(fields, rows, keyed, initial)
    ib = initial[3]
    result = {
        "source_pin": SOURCE, "master_git_blob": git_blob(data),
        "csv_sha256": sha(raw), "gzip_bytes": len(data), "csv_bytes": len(raw),
        "rows": len(rows), "columns": len(fields), "file_paths": 2558, "symbols": 2183,
        "inventory_validated": True, "compared_to": INITIAL,
        "lanes": {}, "next_units": {}, "unkeyed_next": []
    }
    for lane in LANES:
        rr = [r for r in rows if r["Parallel lane"] == lane]
        touched = [r for r in rr if any(v != ib[r["ID"]].get(k, "") for k, v in r.items())]
        result["lanes"][lane] = {
            "total": len(rr), "changed_since_initial": len(touched),
            "untouched_since_initial": len(rr)-len(touched),
            "by_batch": {b: {"total": n, "changed": sum(r["Lane batch"] == b for r in touched)}
                         for (l,b),n in batches.items() if l == lane},
            "resolved_action": tally(rr, "Resolved action"),
            "phase": tally(rr, "Phase decision"),
            "basis": tally(rr, "Review basis"),
            "runtime": tally(rr, "Runtime test")
        }
    for row in rows:
        if row["Phase decision"] != "NEXT":
            continue
        key = row.get("Implementation unit / dedup key", "").strip()
        if not key:
            result["unkeyed_next"].append({"id": row["ID"], "path": row["Repository path"],
                                           "package": row["Next-phase package"]})
            continue
        unit = result["next_units"].setdefault(key, {"count": 0, "lanes": [], "packages": []})
        unit["count"] += 1
        for column, target in [("Parallel lane","lanes"),("Next-phase package","packages")]:
            if row[column] not in unit[target]:
                unit[target].append(row[column])
    result["next_total"] = sum(x["count"] for x in result["next_units"].values()) + len(result["unkeyed_next"])
    return result

def merge(patch_path):
    patch = json.loads(patch_path.read_text(encoding="utf-8"))
    insist(patch["schema"] in ("arbor-transient-stable-id-row-patch/1", "arbor-transient-stable-id-row-patch/2"), "Unknown patch schema")
    insist(patch["source_head"] == SOURCE, "Wrong product source pin")
    base_data = git("show", patch["last_readable_ledger_commit"] + ":" + REL)
    insist(git_blob(base_data) == patch["last_readable_ledger_blob"], "Patch base blob mismatch")
    base = decode(base_data)
    original = MASTER.read_bytes()
    raw, fields, rows, current = decode(original)
    initial = baseline()
    validate(fields, rows, current, initial)
    record_ids = [r["stable_id"] for r in patch["rows"]]
    insist(len(record_ids) == len(set(record_ids)) and 0 < len(record_ids) <= 500, "Patch ID count invalid")
    before = {sid: dict(row) for sid, row in current.items()}
    touched, conflicts = [], []
    for record in patch["rows"]:
        sid = record["stable_id"]
        try:
            old, row = base[3][sid], current[sid]
            insist(all(row[x] == old[x] for x in IDENTITY), "Identity drift")
            insist(row["Repository path"] == record["path"] and row["Symbol / occurrence"] == record["symbol"] and
                   row["Parallel lane"] == record["lane"] == patch["lane"] and
                   row["Lane batch"] == record["batch"], "Patch address mismatch")
            if patch["schema"].endswith("/1"):
                fragment = record["required_existing_evidence_fragment"]
                insist(fragment in "\n".join(old.values()) and fragment in "\n".join(row.values()), "Base evidence marker missing")
                insist(fragment.endswith(record["source_blob_sha"]), "Source identity mismatch")
            else:
                source_sha = record["source_blob_sha"]
                insist(len(source_sha) == 40 and all(c in "0123456789abcdef" for c in source_sha), "Invalid source blob ID")
                insist(record["source_url"].startswith("https://github.com/owenservera/ARBOR/blob/" + SOURCE + "/" + record["path"] + "#L"), "Source URL does not match pinned path")
                insist(bool(record["expected_old_fields"]), "Missing exact base field guards")
            for k,v in record["expected_old_fields"].items():
                insist(old.get(k) == v, "Expected base field " + k)
            changes = {k: str(v) for k,v in record["changed_fields"].items()}
            protected = set(IDENTITY) | {"Inventory HEAD","Inventory origin","Notes","Implementation commit"}
            insist(all(k in fields and k not in protected for k in changes), "Invalid changed fields")
            insist(changes.get("Review HEAD") == SOURCE and changes.get("Runtime test") == "UNRUN", "Unsupported evidence upgrade")
            for k,v in changes.items():
                insist(row[k] == old[k] or row[k] == v, "Concurrent conflict: " + k)
            marker = record["append_evidence"]
            if marker in row["Notes"] and all(row[k] == v for k,v in changes.items()):
                continue
            history = {k: row[k] for k,v in changes.items() if row[k] != v}
            row.update(changes)
            row["Notes"] += "\n[LABS RECONCILE] " + json.dumps({"previous": history, "evidence": marker,
                "source": record["source_url"], "anchors": record["source_anchors"],
                "review_completed": record["review_completed"], "scope": record["evidence_scope"]}, ensure_ascii=False, sort_keys=True)
            touched.append(sid)
        except (KeyError, ValueError) as exc:
            conflicts.append({"id": sid, "reason": str(exc)})
    insist(not conflicts, "No write; conflicts: " + json.dumps(conflicts))
    insist(all(current[sid] == row for sid,row in before.items() if sid not in touched), "Non-target row modified")
    if not touched:
        print(json.dumps({"merged": 0, "already_applied": len(record_ids)}))
        return
    validate(fields, rows, current, initial)
    buf = io.StringIO(newline="")
    writer = csv.DictWriter(buf, fieldnames=fields, lineterminator="\r\n")
    writer.writeheader()
    writer.writerows(rows)
    rewritten = buf.getvalue().encode("utf-8-sig" if raw.startswith(b"\xef\xbb\xbf") else "utf-8")
    zipped = gzip.compress(rewritten, compresslevel=9, mtime=0)
    parsed = decode(zipped)
    insist(parsed[0] == rewritten and parsed[1] == fields and parsed[2] == rows, "Roundtrip mismatch")
    insist(MASTER.read_bytes() == original, "Local master changed while merging")
    with tempfile.NamedTemporaryFile(dir=MASTER.parent, delete=False) as handle:
        path = pathlib.Path(handle.name)
        handle.write(zipped)
    try:
        os.replace(path, MASTER)
    finally:
        path.unlink(missing_ok=True)
    insist(MASTER.read_bytes() == zipped, "Master readback mismatch")
    print(json.dumps({"merged": len(touched), "unchanged_other_rows": len(rows)-len(touched),
                      "before_blob": git_blob(original), "after_blob": git_blob(zipped), "csv_sha256": sha(rewritten)}))

def main():
    p = argparse.ArgumentParser()
    p.add_argument("command", choices=["audit","merge"])
    p.add_argument("patch", nargs="?", type=pathlib.Path)
    args = p.parse_args()
    if args.command == "merge":
        insist(args.patch is not None, "Patch required")
        merge(args.patch)
    print(json.dumps(audit(MASTER.read_bytes()), ensure_ascii=False, indent=2, sort_keys=True))

if __name__ == "__main__":
    main()

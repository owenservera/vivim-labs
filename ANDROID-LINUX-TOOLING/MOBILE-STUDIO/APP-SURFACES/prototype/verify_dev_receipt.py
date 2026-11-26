#!/usr/bin/env python3
"""Verify a user-selected diagnostic receipt; never scan device or chat files."""
import argparse
import hashlib
import json
from pathlib import Path

MAX_RECEIPT=65536

def verify(receipt, expected_sha, expected_version):
    checks={}
    checks["schema"]=receipt.get("schema")=="vivim.android.developer-receipt/1"
    checks["source_claim"]=receipt.get("source")=="apk-process"
    checks["no_chat_content_claim"]=receipt.get("contains_private_chat_content") is False
    def value(key):
        p=receipt.get(key,{})
        return p.get("value",{}) if isinstance(p,dict) and p.get("status")=="observed" else {}
    identity=value("installed_identity")
    checks["package"]=identity.get("package")=="live.vivim.surfaceprobe"
    checks["version"]=identity.get("version_code")==expected_version
    checks["artifact_digest"]=identity.get("base_apk_sha256")==expected_sha
    jni=value("native_jni")
    checks["jni"]=jni.get("checks")==[True]*5 and jni.get("all_passed") is True
    checks["cache"]=value("own_cache_roundtrip").get("exact_match") is True
    contracts=value("whatsapp_contracts")
    checks["whatsapp_intent_resolution"]=contracts.get("launcher_resolved") is True and contracts.get("text_share_resolved") is True and contracts.get("effect_attempted") is False
    checks["no_internet_permission"]=value("permission_state").get("internet_declared") is False
    return {"checks":checks,"all_passed":all(checks.values()),"evidence":"owner-supplied receipt; consistency checked, not cryptographic runtime attestation"}

def main():
    p=argparse.ArgumentParser()
    p.add_argument("receipt",type=Path,help="Explicitly selected diagnostic JSON")
    p.add_argument("apk",type=Path,help="Exact expected signed APK")
    p.add_argument("--version",type=int,required=True)
    a=p.parse_args()
    with a.receipt.open("rb") as f:
        raw=f.read(MAX_RECEIPT+1)
    if len(raw)>MAX_RECEIPT: raise SystemExit("Receipt exceeds bound")
    with a.apk.open("rb") as f:
        digest=hashlib.file_digest(f,"sha256").hexdigest()
    data=json.loads(raw)
    if not isinstance(data,dict): raise SystemExit("Invalid receipt object")
    result=verify(data,digest,a.version)
    print(json.dumps(result,indent=2))
    return 0 if result["all_passed"] else 1

if __name__=="__main__":
    raise SystemExit(main())

#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
: "${ANDROID_JAR:?Set ANDROID_JAR to verified API 35 platform android.jar}"
test -f "$ANDROID_JAR"
umask 077
rm -rf out/classes out/dex
mkdir -p out/classes out/dex out/jni
rustc --edition=2021 --crate-type lib --emit=obj -C opt-level=z -C panic=abort -C relocation-model=pic native_probe.rs -o out/jni/probe.o
ld.lld -shared --no-undefined -soname libvivim_probe.so -z max-page-size=16384 -z noexecstack -o out/jni/libvivim_probe.so out/jni/probe.o
llvm-strip --strip-unneeded out/jni/libvivim_probe.so
if llvm-readelf -d out/jni/libvivim_probe.so | grep -E "RUNPATH|RPATH|NEEDED"; then exit 1; fi
aapt2 link -I "$ANDROID_JAR" --manifest AndroidManifest.xml -o out/unsigned.apk
javac --release 8 -cp "$ANDROID_JAR" -d out/classes MainActivity.java FixtureActivity.java WhatsAppActivity.java SnapshotService.java OwnerActivity.java WhatsAppNoticeService.java ReviewedActivity.java
d8 --lib "$ANDROID_JAR" --min-api 26 --output out/dex out/classes/live/vivim/surfaceprobe/*.class
python -c 'import zipfile; z=zipfile.ZipFile("out/unsigned.apk","a",compression=zipfile.ZIP_DEFLATED); z.write("out/dex/classes.dex","classes.dex"); z.write("out/jni/libvivim_probe.so","lib/arm64-v8a/libvivim_probe.so"); z.close()'
# Compressed ARM64 library is explicitly extracted by Android. ELF LOAD alignment is 16 KiB.
input=out/unsigned.apk
if command -v zipalign >/dev/null; then zipalign -f 4 "$input" out/aligned.apk; input=out/aligned.apk; fi
key=out/probe-debug.p12
if [ ! -f "$key" ]; then
  keytool -genkeypair -keystore "$key" -storetype PKCS12 -storepass android -keypass android -alias probe -keyalg RSA -keysize 2048 -validity 365 -dname "CN=VIVIM Local Research Fixture"
fi
# Debug fixture key only, never production.
apksigner sign --ks "$key" --ks-pass pass:android --ks-key-alias probe --out out/vivim-surface-probe.apk "$input"
apksigner verify --verbose out/vivim-surface-probe.apk
sha256sum out/vivim-surface-probe.apk
ls -l out/vivim-surface-probe.apk

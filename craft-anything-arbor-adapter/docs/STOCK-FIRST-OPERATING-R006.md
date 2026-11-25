# Craft Anything — stock-first operating contract R006
Owner decision 2026-11-25: use Craft 0.6.1 exactly as delivered, with only the necessary local setup/integration. Do not fork or migrate Craft, build an alternate browser controller, or require Studio changes before using it. Shape exported evidence downstream.

## Native startup (owner local PowerShell)
$Craft = 'C:\0-BlackBoxProject-0\craft-anything\target\release\craft-anything-cli.exe'
& $Craft --version
& $Craft doctor
& $Craft setup --dry-run --client claude_code

Review the setup preview. If Claude Code is installed and the owner wishes to use it, run the stock setup locally (without --yes), then use its normal sign-in. This is a local AI option, not a requirement to use an API key. Read the doctor report before choosing other detected clients. Craft inspection tools themselves need no LLM key. Do not change global provider credentials.

## Website sessions
Use stock local app or owner-operated local MCP client with the existing executable, Clean room mode, and a distinct project per target. Stock restricted shared-host MCP expressly refuses saved sign-ins even with allow-web/allow-capture; do not circumvent it or expose unrestricted local tools through the remote tunnel. For authenticated investigation use the supported owner-operated local application/client sign-in workflow, subject to the applicable site authorization and terms. Use web_site_sign_in with URL, let the owner sign in in its visible browser and close the window; Craft saves that exact host/port sign-in locally. Never read/copy existing Studio Chrome profile or expose credentials. web_site_open with signed_in:true subsequently requires a saved sign-in; verify the actual page is authenticated, not just the flag. Keep open/capture/export in one persistent session. Never use --allow-run indiscriminately. Public first-party smoke can use stock restricted MCP with allow-web and allow-capture; live third-party active actions not authorized by capture scope.

Targets: https://chatgpt.com/ ; https://claude.ai/ ; https://gemini.google.com/app ; https://www.facebook.com/ . Capture all permitted reachable UI incl settings, nested menus, dialogs, privacy/account controls, responsive states, recovery, and app-specific surfaces per R005. Never change settings, send, purchase, delete, or disclose private content during discovery. Local raw evidence only. Cross-origin account pages separately scoped. Record unknowns and exclusions.

## Downstream only
Craft capture and original evidence export -> native verifier -> existing Labs adapter import -> source baseline reconciliation -> existing Studio R003 read-only atlas endpoints. Harness consumes proposals to improve existing provider semantics; VIVIM HQKG/VLANG receive only accepted typed scope-qualified read mappings. Host owns all effects, approvals and receipts. Do not add second product renderer or graph. No raw browser data in Git. Existing baselines chatgpt, claude, gemini; Facebook source gap is explicitly unknown.

## Activation state
Craft 0.6.1 compiled previously; native VIVIM scratch directory created with target subdirectories. Current VIVIM grant still arbor/scratch only; Craft/Labs executable access not granted and previous general terminal probe denied/uncertain. No live website capture or native verifier run in this round. Local owner can run startup above now; native workstation owner must extend authorized access before remote execution. No Desktop Commander fallback.

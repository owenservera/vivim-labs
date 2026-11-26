# R002 — Native Android development provisioning
2026-11-26. Actual device evidence; no credentials or private inventory published.

GitHub CLI 2.105.0 installed from Termux ARM64 repository. Owner completed official device authorization. Git credential helper configured; authenticated repository read and shallow main clone succeeded at Product commit 2f00ccc2b0d05ada83103396cc14e21ab8700ee8. Push not tested. CLI reported plaintext credential storage; private config mode 600 verified. No token values recorded.

Native package installation exited 0: 23 new packages including tmux 3.8, ripgrep 16.0.0, jq 1.8.2, sqlite 3.54.1, sccache 0.20.1, just 1.59.0, proot-distro 5.10.3, fd 10.5.2, bat 0.26.1, fzf 0.75.0, tree 2.3.2, shellcheck 0.11.0, rust-analyzer package 20261123, rust-src 1.100.0, git-lfs 3.8.1, rsync 3.5.2. Estimated installed size 214 MB. No existing packages upgraded. No llama.cpp, model weights, or guest Linux distribution installed.

Local launcher: [dev-run.sh](experiments/dev-run.sh), final source commit 11c7fa13d880d27603824d3c312f30bccf7f4b90. Scoped to Product checkout; one Cargo job/test thread, debug info and incremental disabled, optional sccache 512 MiB. Verified Bash syntax, ShellCheck, locked Cargo metadata, clean Git tree. Native jq and SQLite query passed. Git LFS initialized only in local checkout. Isolated tmux session created, verified and closed. sccache server start/stats/stop verified; compiler cache effectiveness not yet tested. No persistent cache server started. Initial lint warning corrected; one output-pipe broken pipe avoided in subsequent full-output check.

Full locked Cargo workspace check launched; final result not established when this provisioning evidence was recorded. Separate from product acceptance. No production source changes.

# Progress: Optimised Release Build

Make an optimised binary of the Security Doc RAG agent. Use Rig only. Do not change the tech stack.

Writing rules (ASD-STE100): use short sentences. Use the active voice. Give one instruction in each step.
Mark a step `[x]` only after you do the step and you verify it.

## Phase 1: Before you change anything

- [x] 1.1 Check that the git tree is clean. (Last commit `5323bc5` was clean. Only `Cargo.toml` and `progress.md` have changes now.)
- [x] 1.2 Record the debug binary size. (21080064 bytes, about 20.1 MiB.)
- [x] 1.3 Record the dependency list. Make sure `rig`, `anyhow`, `serde` and `tokio` are the only direct dependencies. (Verified in `Cargo.toml`.)

## Phase 2: Release profile

- [x] 2.1 Add `[profile.release]` to `Cargo.toml`.
- [x] 2.2 Set `opt-level = 3`.
- [x] 2.3 Set `lto = "fat"`.
- [x] 2.4 Set `codegen-units = 1`.
- [x] 2.5 Set `panic = "abort"`.
- [x] 2.6 Set `strip = true`.
- [x] 2.7 Do not add a new dependency.

## Phase 3: Build

- [x] 3.1 Run `cargo build --release`.
- [x] 3.2 Make sure the build has no error and no warning. (No warning in the output.)
- [x] 3.3 Record the release binary size. (6782464 bytes, about 6.5 MiB. Build time 5 min 38 s.)
- [x] 3.4 Run `cargo test --release`. Make sure all tests pass. (15 passed.)
- [x] 3.5 Run `cargo clippy --release`. Make sure it has no warning.

## Phase 4: Test the binary

- [x] 4.1 Run the binary with no argument. Make sure it lists the document types and exits with code 2. (Exit code 2.)
- [x] 4.2 Run the binary with a wrong document type. Make sure it exits with code 2. (Exit code 2.)
- [x] 4.3 Run the binary with `hardening-checklist "release build check"`. (Ran `security-doc-rag.exe` directly.)
- [x] 4.4 Make sure the binary creates `output/hardening-checklist-release-build-check.md`. (10715 bytes.)
- [x] 4.5 Make sure the file has all sections of the document type. (6 of 6 sections.)

## Phase 5: Finish

- [x] 5.1 Make sure `Cargo.toml` has no new dependency. (Rig, anyhow, serde and tokio only.)
- [x] 5.2 Update `README.md` with the release build command and the binary path.
- [x] 5.3 Make a git commit.

## Acceptance

- [x] The release binary is smaller than the debug binary. (6.5 MiB against 20.1 MiB.)
- [x] Rig is the only LLM framework.
- [x] The binary creates a complete Markdown document.

## Phase 6: Match the layout of the other project

- [x] 6.1 Make `target/` a directory junction to `D:\cargo-target-bowfin`. (Drive C: has little free space.)
- [x] 6.2 Make sure Git ignores `target/`. (`git check-ignore` confirms it.)
- [x] 6.3 Run `cargo build` and `cargo build --release` with no `CARGO_TARGET_DIR`.
- [x] 6.4 Make sure the .exe files are in the project: `target/debug/security-doc-rag.exe` (21080064 bytes) and `target/release/security-doc-rag.exe` (6782464 bytes).

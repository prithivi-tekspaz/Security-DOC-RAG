# Progress: Security DOC RAG Build and Layout

Build the Security DOC RAG agent in the same way as the reference project `D:\Git\security-rag`. Use Rig only. Do not change the tech stack.

Writing rules (ASD-STE100): use short sentences. Use the active voice. Give one instruction in each step.
Mark a step `[x]` only after you do the step and you verify it.

## Phase 1: Study the reference project

- [x] 1.1 Open `D:\Git\security-rag`. Read the file list. (Read-only. No file changed.)
- [x] 1.2 Note that `target/` is a real folder inside the project. It holds `CACHEDIR.TAG` and `debug/`. It is 1.5 GB.
- [x] 1.3 Note that `target/debug/` holds `build`, `deps`, `examples`, `incremental`, the `.d` file, the `.exe` and the `.pdb`.
- [x] 1.4 Note that the project is on drive D:, not on drive C:.
- [x] 1.5 Note that `.gitignore` has the line `/target`.
- [x] 1.6 Note that `Cargo.toml` has `publish = false` and `[lints.rust] unsafe_code = "forbid"`.
- [x] 1.7 Note the dependencies: `rig` 0.44 (`derive`, `pdf`), `anyhow`, `serde`, `tokio`. This project has the same dependencies.

## Phase 2: Place the project

- [x] 2.1 Find the project folder. It is `D:\Git\Security Doc RAG`. This folder is the main repository of the `bowfin` worktree.
- [x] 2.2 Make sure the folder is on drive D:. Drive D: has 39 GB free.
- [x] 2.3 Fast-forward `master` to the finished work. (Commit `45415c3`.)
- [x] 2.4 Make sure the folder holds `src/main.rs`, `src/sections.rs`, `Cargo.toml`, `Cargo.lock`, `README.md`, `docs/`, `progress-tracker.md` and the plan.
- [x] 2.5 Copy the ten generated documents into `output/`.
- [x] 2.6 Make sure no junction is in use. `target/` must be a real folder.

## Phase 3: Match the reference conventions

- [x] 3.1 Change `.gitignore` to the form `/target` and `/output`.
- [x] 3.2 Add `publish = false` to `Cargo.toml`.
- [x] 3.3 Add `[lints.rust] unsafe_code = "forbid"` to `Cargo.toml`. Keep `#![forbid(unsafe_code)]` in `src/main.rs`.
- [x] 3.4 Keep `[profile.release]` in `Cargo.toml`. (The reference project has no release profile.)
- [x] 3.5 Keep the package name `security-doc-rag`. The `.exe` name stays `security-doc-rag.exe`.
- [x] 3.6 Make sure `Cargo.toml` has no new dependency.

## Phase 4: Build inside the project

- [x] 4.1 Run `cargo build` in `D:\Git\Security DOC RAG`. Do not set `CARGO_TARGET_DIR`.
- [x] 4.2 Make sure `target/debug/security-doc-rag.exe` exists.
- [x] 4.3 Make sure `target/debug/` has `build`, `deps`, `examples`, `incremental`, the `.d` file and the `.pdb`.
- [x] 4.4 Run `cargo build --release`.
- [x] 4.5 Make sure `target/release/security-doc-rag.exe` exists.
- [x] 4.6 Make sure the build has no error and no warning.
- [x] 4.7 Record the size of each `.exe`. (Debug 21080064 bytes, about 20 MiB. Release 6782464 bytes, about 6.5 MiB. `target/` is 2.3 GB.)

## Phase 5: Test

- [x] 5.1 Run `cargo test`. Make sure all 15 tests pass.
- [x] 5.2 Run `cargo clippy --all-targets`. Make sure it has no warning.
- [x] 5.3 Run the `.exe` with no argument. Make sure it lists the document types and exits with code 2.
- [x] 5.4 Run `target/release/security-doc-rag.exe hardening-checklist "layout check"` from the project root. (Done.)
- [x] 5.5 Make sure the run creates `output/hardening-checklist-layout-check.md` with all six sections.

## Phase 6: Finish

- [x] 6.1 Update `README.md`. Remove the junction note. Show the real `target/` paths.
- [x] 6.2 Make a git commit on `master`.
- [x] 6.3 Make sure `git status` shows no untracked file except ignored folders.

## Earlier work (done)

- [x] E.1 Build the Rig agent: ingestion, `sections.rs`, agent, CLI and output. (See `progress-tracker.md`.)
- [x] E.2 Generate each of the five document types for two topics.
- [x] E.3 Add the optimised `[profile.release]`: `opt-level = 3`, fat LTO, one codegen unit, `panic = "abort"`, `strip = true`.

## Acceptance

- [x] `target/` is a real folder inside `D:\Git\Security DOC RAG`.
- [x] The debug and the release `.exe` are inside `target/`.
- [x] Rig is the only LLM framework in `Cargo.toml`.
- [x] The binary creates a complete Markdown document.

## Phase 7: Ask for a specific document

- [x] 7.1 Add the `--instructions "<text>"` flag. The agent puts the text in the prompt of every section.
- [x] 7.2 Show the instructions in the document header.
- [x] 7.3 Add the `ask "<request>"` mode. The agent picks the document type from words in the request.
- [x] 7.4 Exit with code 2 and show the usage text when the request has no document type.
- [x] 7.5 Limit the file name slug to 60 characters.
- [x] 7.6 Add unit tests for argument parsing, type detection and the prompt. `cargo test` shows 20 passed.
- [x] 7.7 Run `cargo clippy --all-targets`. No warning.
- [x] 7.8 Run `ask "Write a threat model for a payment API that stores card tokens. Focus on PCI-DSS."` on the real model. The file has 6 sections. It mentions PCI 26 times.
- [x] 7.9 Run `hardening-checklist "PostgreSQL 16" --instructions "Target CIS level 2. Write short bullet items only."` on the real model. The file has 6 sections, 50 bullet lines and the instructions in the header.
- [x] 7.10 Update `README.md` with both modes.

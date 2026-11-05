# Progress Tracker: Security Doc RAG

Build a local Rust agent. The agent writes security documents. Use the Rig framework only.
Source of the plan: `SECURITY_DOC_RAG_PLAN.md`.

Writing rules (ASD-STE100): use short sentences. Use the active voice. Give one instruction in each step.
Mark a step `[x]` only after you do the step and you verify it.

Owner key: **LEAD** = main session. **A** = sections agent. **B** = main.rs agent. **C** = docs agent.

## Phase 1: Project setup

- [x] 1.1 Read `SECURITY_DOC_RAG_PLAN.md`. (LEAD)
- [x] 1.2 Read the old `src/main.rs` chatbot. (LEAD)
- [x] 1.3 Make a git commit before you change code. (LEAD)
- [x] 1.4 Make `Cargo.toml` with `rig`, `anyhow`, `serde`, `tokio`. Add no other dependency. (LEAD)
- [x] 1.5 Make `.gitignore` with `target/` and `output/`. (LEAD)
- [x] 1.6 Make the `docs/` folder and the `output/` folder. (LEAD)
- [ ] 1.7 Check that Rig 0.44 `AgentBuilder` accepts `.temperature()`, `.max_tokens()` and `.additional_params()`. (B)
- [ ] 1.8 Add `#![forbid(unsafe_code)]` to the crate. (B)
- [ ] 1.9 Pull the `nomic-embed-text` model with Ollama. (USER)
- [ ] 1.10 Pull the `CyberCrew/notmythos-8b:latest` model with Ollama. (USER)
- [ ] 1.11 Run `ollama show` on the model. Record the real context length. (USER)

## Phase 2: Ingestion and index

- [ ] 2.1 Keep `chunk_text` with a limit of 2000 characters on word boundaries. (B)
- [ ] 2.2 Keep the six `chunk_text` unit tests. (B)
- [ ] 2.3 Load `docs/*.pdf` with `rig::loaders::PdfFileLoader`. (B)
- [ ] 2.4 Use the built-in security corpus when `docs/` is missing or empty. (B)
- [ ] 2.5 Make a `Document { id, content }` for each chunk. (B)
- [ ] 2.6 Embed each chunk with `EmbeddingsBuilder` and `nomic-embed-text`. (B)
- [ ] 2.7 Put the embeddings in an `InMemoryVectorStore`. (B)
- [ ] 2.8 Print `Loaded N security chunks.` and `Successfully generated embeddings`. (B)

## Phase 3: Section plans

- [ ] 3.1 Make `src/sections.rs`. (A)
- [ ] 3.2 Define the `Section` type with a title and a one-line instruction. (A)
- [ ] 3.3 Define the `DocType` type with a key, a title and a section list. (A)
- [ ] 3.4 Define `threat-model`: Scope, Assets, Trust Boundaries, Threats (STRIDE), Mitigations, Residual Risk. (A)
- [ ] 3.5 Define `secure-coding-guideline`. (A)
- [ ] 3.6 Define `incident-response-runbook`. (A)
- [ ] 3.7 Define `hardening-checklist`. (A)
- [ ] 3.8 Define `vulnerability-remediation-report`. (A)
- [ ] 3.9 Add a `find` function that looks up a document type by key. (A)
- [ ] 3.10 Add a `slugify` function for output file names. (A)

## Phase 4: Documentation agent

- [ ] 4.1 Write the documentation preamble from plan stage 7. (B)
- [ ] 4.2 Build the agent with `AgentBuilder` and `dynamic_context(3, index)`. (B)
- [ ] 4.3 Set the temperature to 0.35. (B)
- [ ] 4.4 Set `num_ctx` to 8192 with `.additional_params()`. (B)
- [ ] 4.5 Write one short prompt for each section: doc type, topic, section title, instruction. (B)
- [ ] 4.6 Call the agent one time for each section. (B)
- [ ] 4.7 Join the sections in Rust. Add a header with type, topic, date and model. (B)

## Phase 5: CLI and output

- [ ] 5.1 Parse the arguments `<doc-type> <topic>`. (B)
- [ ] 5.2 Show the valid document types when an argument is wrong. (B)
- [ ] 5.3 Print `[i/n] Title ... done` for each section. (B)
- [ ] 5.4 Write `output/<type>-<topic>.md` with `std::fs`. (B)
- [ ] 5.5 Print `Saved output/<file>.md`. (B)

## Phase 6: Test and tune

- [ ] 6.1 Add unit tests for the section plans. Each plan must have sections. (A)
- [ ] 6.2 Add unit tests for the file name slugs. (A)
- [ ] 6.3 Run `cargo build`. Make sure it has no error and no warning. (B)
- [ ] 6.4 Run `cargo test`. Make sure all tests pass. (B)
- [ ] 6.5 Run `cargo clippy`. Make sure it has no warning. (B)
- [ ] 6.6 Write `README.md` with setup, run steps and the review note. (C)
- [ ] 6.7 Generate each of the five types for two topics. (USER)
- [ ] 6.8 Tune chunk size, top-N and temperature. (USER)

## Acceptance

- [ ] Rig is the only LLM framework in `Cargo.toml`.
- [ ] No external API key, telemetry or cloud service is in use.
- [ ] `cargo run -- threat-model "payment API"` writes `output/threat-model-payment-api.md`.
- [ ] Model-only lines have the mark `(model knowledge)`. Uncertain lines say `needs review`.
- [ ] The README warns that a human must review each draft.

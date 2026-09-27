---
okf_version: "0.2"
type: Function
title: history
description: Per-primitive git history.
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/history
language: rust
---

# history

Per-primitive git history.

## Signature

```rust
fn history(&self, _primitive_path: &Path) -> Result<Vec<HistoryEntry>, LibraryError>
```

## Docstring

Per-primitive git history.

Returns up to 50 entries from `git log --follow --max-count 50
-- <primitive_path>`, newest first. `primitive_path` may be
absolute or relative to [`Self::root_dir`]; absolute paths
outside `root_dir` are rejected with
[`LibraryError::NotFound`].

Per `v0.9-snxlib-as-file-plan.md` §3 this is the *single*
hook the SCH Library / Footprint / Sim editors and the
Library Browser tab call to populate the
`oxide_widgets::history_pane::HistoryPane` widget — there's
no second code path. Adapters that aren't backed by git
(e.g. the database adapter) keep the default
`Backend("history not implemented")` response so the UI can
gracefully degrade to "history unavailable" without aborting.

## Source
Lines 505–509 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |

---
okf_version: "0.2"
type: Function
title: persist_project_by_id
description: "Materialise any pending libraries, then write the identified"
resource: crates/oxide-app/src/app/handlers/document_files/save.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/save/persist_project_by_id
language: rust
---

# persist_project_by_id

Materialise any pending libraries, then write the identified

## Signature

```rust
impl Oxide { fn persist_project_by_id(&mut self, project_id: crate::app::state::ProjectId) -> Result<()> }
```

## Docstring

Materialise any pending libraries, then write the identified
project's `.snxprj` to disk. On success clears the project's
dirty marker, auto-commits to the project git repo, and
refreshes the panel context. Returns the *write* failure (the
per-library materialise errors are logged and the entry re-
stashed for retry, matching the historical behaviour) so callers
can surface a real save failure instead of swallowing it.

## Source
Lines 160–261 in `crates/oxide-app/src/app/handlers/document_files/save.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [save](/crates/oxide-app/src/app/handlers/document_files/save.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [materialize_pending_library](/crates/oxide-app/src/library/commands/materialize_pending_library.md) |
| calls | [write_project](/crates/oxide-app/tests/measure_library_open/write_project.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |

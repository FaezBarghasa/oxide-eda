---
okf_version: "0.2"
type: Function
title: handle_annotate
resource: crates/oxide-app/src/app/handlers/erc/annotate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/annotate/handle_annotate
language: rust
---

# handle_annotate

## Signature

```rust
impl Oxide { pub(crate) fn handle_annotate(&mut self, mode: oxide_engine::AnnotateMode) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 8–173 in `crates/oxide-app/src/app/handlers/erc/annotate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate](/crates/oxide-app/src/app/handlers/erc/annotate.md) |
| calls | [assemble_active_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_active_project_sheets.md) |
| calls | [ordered_project_sheet_paths](/crates/oxide-app/src/app/project_sheets/ordered_project_sheet_paths.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |

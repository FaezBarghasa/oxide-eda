---
okf_version: "0.2"
type: Function
title: handle_add_existing_file_picked
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/add/handle_add_existing_file_picked
language: rust
---

# handle_add_existing_file_picked

## Signature

```rust
impl Oxide { pub(crate) fn handle_add_existing_file_picked(
        &mut self,
        project_idx: usize,
        paths: Option<Vec<std::path::PathBuf>>,
    ) }
```

## Visibility

- `pub(crate)`

## Source
Lines 239–291 in `crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [add](/crates/oxide-app/src/app/handlers/dock/project_navigation/add.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |

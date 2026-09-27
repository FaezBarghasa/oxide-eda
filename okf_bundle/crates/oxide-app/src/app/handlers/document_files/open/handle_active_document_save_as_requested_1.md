---
okf_version: "0.2"
type: Function
title: handle_active_document_save_as_requested
resource: crates/oxide-app/src/app/handlers/document_files/open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/open/handle_active_document_save_as_requested_1
language: rust
---

# handle_active_document_save_as_requested

## Signature

```rust
pub(crate) fn handle_active_document_save_as_requested(&mut self, path: PathBuf)
```

## Visibility

- `pub(crate)`

## Source
Lines 137–141 in `crates/oxide-app/src/app/handlers/document_files/open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open](/crates/oxide-app/src/app/handlers/document_files/open.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |

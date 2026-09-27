---
okf_version: "0.2"
type: Function
title: save_active_document_as
resource: crates/oxide-app/src/app/handlers/document_files/save.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/save/save_active_document_as_1
language: rust
---

# save_active_document_as

## Signature

```rust
pub(crate) fn save_active_document_as(&mut self, path: PathBuf) -> Result<()>
```

## Visibility

- `pub(crate)`

## Source
Lines 263–271 in `crates/oxide-app/src/app/handlers/document_files/save.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [save](/crates/oxide-app/src/app/handlers/document_files/save.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |

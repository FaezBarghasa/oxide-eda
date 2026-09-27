---
okf_version: "0.2"
type: Function
title: save_active_document
resource: crates/oxide-app/src/app/handlers/document_files/save.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/save/save_active_document
language: rust
---

# save_active_document

## Signature

```rust
impl Oxide { pub(crate) fn save_active_document(&mut self) -> Result<iced::Task<Message>> }
```

## Visibility

- `pub(crate)`

## Source
Lines 10–96 in `crates/oxide-app/src/app/handlers/document_files/save.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [save](/crates/oxide-app/src/app/handlers/document_files/save.md) |
| calls | [spawn_save_as_for_new_primitive](/crates/oxide-app/src/app/handlers/document_files/mod/spawn_save_as_for_new_primitive.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
| calls | [context](/crates/oxide-app/src/app/handlers/menu/export/tests/context.md) |

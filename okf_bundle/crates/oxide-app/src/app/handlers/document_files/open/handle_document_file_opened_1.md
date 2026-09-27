---
okf_version: "0.2"
type: Function
title: handle_document_file_opened
resource: crates/oxide-app/src/app/handlers/document_files/open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/open/handle_document_file_opened_1
language: rust
---

# handle_document_file_opened

## Signature

```rust
pub(crate) fn handle_document_file_opened(
        &mut self,
        path: Option<PathBuf>,
    ) -> iced::Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 10–28 in `crates/oxide-app/src/app/handlers/document_files/open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open](/crates/oxide-app/src/app/handlers/document_files/open.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |

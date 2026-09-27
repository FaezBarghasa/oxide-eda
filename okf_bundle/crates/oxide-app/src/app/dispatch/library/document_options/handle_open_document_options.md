---
okf_version: "0.2"
type: Function
title: handle_open_document_options
description: Tools menu fired Document Options for the library at
resource: crates/oxide-app/src/app/dispatch/library/document_options.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/document_options/handle_open_document_options
language: rust
---

# handle_open_document_options

Tools menu fired Document Options for the library at

## Signature

```rust
impl Oxide { pub(super) fn handle_open_document_options(
        &mut self,
        library_path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Tools menu fired Document Options for the library at
`library_path` — opens the modal pre-filled with its display
settings.

## Source
Lines 13–25 in `crates/oxide-app/src/app/dispatch/library/document_options.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document_options](/crates/oxide-app/src/app/dispatch/library/document_options.md) |

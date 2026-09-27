---
okf_version: "0.2"
type: Function
title: handle_document_options_set_sheet_color
description: Modal — pick a new sheet color preset.
resource: crates/oxide-app/src/app/dispatch/library/document_options.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/document_options/handle_document_options_set_sheet_color
language: rust
---

# handle_document_options_set_sheet_color

Modal — pick a new sheet color preset.

## Signature

```rust
impl Oxide { pub(super) fn handle_document_options_set_sheet_color(
        &mut self,
        c: crate::panels::SheetColor,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Modal — pick a new sheet color preset.

## Source
Lines 28–36 in `crates/oxide-app/src/app/dispatch/library/document_options.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document_options](/crates/oxide-app/src/app/dispatch/library/document_options.md) |

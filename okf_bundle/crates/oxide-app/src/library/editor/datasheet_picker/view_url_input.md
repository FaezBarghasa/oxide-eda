---
okf_version: "0.2"
type: Function
title: view_url_input
description: ─────────────────────────────────────────────────────────────────────
resource: crates/oxide-app/src/library/editor/datasheet_picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/datasheet_picker/view_url_input
language: rust
---

# view_url_input

─────────────────────────────────────────────────────────────────────

## Signature

```rust
fn view_url_input(
    datasheet: Option<&'a DatasheetRef>,
    address: EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

─────────────────────────────────────────────────────────────────────
Per-mode rows
─────────────────────────────────────────────────────────────────────

## Source
Lines 111–132 in `crates/oxide-app/src/library/editor/datasheet_picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [datasheet_picker](/crates/oxide-app/src/library/editor/datasheet_picker.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/datasheet_picker/view.md) |

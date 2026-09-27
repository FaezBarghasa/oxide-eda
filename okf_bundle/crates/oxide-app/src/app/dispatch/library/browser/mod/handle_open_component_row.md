---
okf_version: "0.2"
type: Function
title: handle_open_component_row
description: "Open the Component Preview tab for `(library_path, table, row_id)`."
resource: crates/oxide-app/src/app/dispatch/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/mod/handle_open_component_row
language: rust
---

# handle_open_component_row

Open the Component Preview tab for `(library_path, table, row_id)`.

## Signature

```rust
impl Oxide { pub(super) fn handle_open_component_row(
        &mut self,
        library_path: std::path::PathBuf,
        table: String,
        row_id: RowId,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Open the Component Preview tab for `(library_path, table, row_id)`.
Re-uses the existing tab if one is already open.

## Source
Lines 748–825 in `crates/oxide-app/src/app/dispatch/library/browser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [browser](/crates/oxide-app/src/app/dispatch/library/browser/mod.md) |

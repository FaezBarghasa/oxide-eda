---
okf_version: "0.2"
type: Function
title: handle_bom_preview_toggle_column
description: "Flip a column's presence in `BomOptions.columns`. Removing a"
resource: crates/oxide-app/src/app/handlers/menu/export/bom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/bom/handle_bom_preview_toggle_column
language: rust
---

# handle_bom_preview_toggle_column

Flip a column's presence in `BomOptions.columns`. Removing a

## Signature

```rust
impl Oxide { pub(crate) fn handle_bom_preview_toggle_column(&mut self, col: BomColumn) }
```

## Visibility

- `pub(crate)`

## Docstring

Flip a column's presence in `BomOptions.columns`. Removing a
column drops it from the preview + export; adding pushes it
to the end of the display order. Re-adding a column the user
previously removed lands it at the end (predictable;
preserving the original slot would require a separate
"available columns" Vec).

## Source
Lines 101–111 in `crates/oxide-app/src/app/handlers/menu/export/bom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-app/src/app/handlers/menu/export/bom.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |

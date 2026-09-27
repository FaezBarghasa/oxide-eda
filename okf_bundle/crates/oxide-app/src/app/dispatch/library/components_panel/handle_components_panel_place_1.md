---
okf_version: "0.2"
type: Function
title: handle_components_panel_place
description: "\"Place into Schematic\" on a Components Panel row. Stage 9 stub —"
resource: crates/oxide-app/src/app/dispatch/library/components_panel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/components_panel/handle_components_panel_place_1
language: rust
---

# handle_components_panel_place

"Place into Schematic" on a Components Panel row. Stage 9 stub —

## Signature

```rust
pub(super) fn handle_components_panel_place(
        &mut self,
        library_path: std::path::PathBuf,
        table: String,
        row_id: RowId,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

"Place into Schematic" on a Components Panel row. Stage 9 stub —
routes through the existing place handler.

## Source
Lines 156–171 in `crates/oxide-app/src/app/dispatch/library/components_panel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/app/dispatch/library/components_panel.md) |

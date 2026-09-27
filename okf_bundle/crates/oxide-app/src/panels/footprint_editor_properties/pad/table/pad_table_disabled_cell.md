---
okf_version: "0.2"
type: Function
title: pad_table_disabled_cell
description: v0.20 — disabled / read-only cell. Shows a value with greyed
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_disabled_cell
language: rust
---

# pad_table_disabled_cell

v0.20 — disabled / read-only cell. Shows a value with greyed

## Signature

```rust
pub(super) fn pad_table_disabled_cell(
    value: impl Into<String>,
    muted: Color,
    border_c: Color,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.20 — disabled / read-only cell. Shows a value with greyed
chrome but no input handler. Used for column placeholders that
don't yet have a backing field (e.g. "0%" in the PASTE table
where percentage overrides aren't wired yet).

## Source
Lines 147–167 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table.md) |

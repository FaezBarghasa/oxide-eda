---
okf_version: "0.2"
type: Function
title: pad_table_static_cell
description: "v0.20 — static text cell. No chrome, just dim text — used for"
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_static_cell
language: rust
---

# pad_table_static_cell

v0.20 — static text cell. No chrome, just dim text — used for

## Signature

```rust
pub(super) fn pad_table_static_cell(
    value: impl Into<String>,
    muted: Color,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.20 — static text cell. No chrome, just dim text — used for
columns like "Rule Expansion" that aren't yet user-editable
(a v0.21 follow-up adds the per-rule override picker).

## Source
Lines 172–180 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table.md) |

---
okf_version: "0.2"
type: Class
title: BomPreviewState
description: Live BOM preview state — the rolled-up table for the active project
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/BomPreviewState
language: rust
---

# BomPreviewState

Live BOM preview state — the rolled-up table for the active project

## Signature

```rust
pub struct BomPreviewState
```

## Visibility

- `pub`

## Docstring

Live BOM preview state — the rolled-up table for the active project
plus the user-editable options that drive the next rollup. Re-rolled
whenever an option toggle fires.

## Methods

- `options`
- `table`
- `variants`
- `sort`
- `column_drag`
- `column_drag_press_x`
- `column_hover`
- `column_widths`
- `column_resize`
- `sidebar_tab`

## Source
Lines 467–508 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |

---
okf_version: "0.2"
type: Function
title: read_grid_style_pref
description: "Read the schematic visible-grid `grid_style` preference. Defaults"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_grid_style_pref
language: rust
---

# read_grid_style_pref

Read the schematic visible-grid `grid_style` preference. Defaults

## Signature

```rust
pub fn read_grid_style_pref() -> GridStyle
```

## Visibility

- `pub`

## Docstring

Read the schematic visible-grid `grid_style` preference. Defaults
to `Dots` (matches the previous hard-coded behaviour).

## Source
Lines 547–549 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_grid_style_pref_at](/crates/oxide-app/src/fonts/mod/read_grid_style_pref_at.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |

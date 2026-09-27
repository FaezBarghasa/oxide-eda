---
okf_version: "0.2"
type: Function
title: read_symbol_grid_size_mm_pref
description: Read the default symbol-editor grid size (mm). Falls back to 1.27 mm.
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_symbol_grid_size_mm_pref
language: rust
---

# read_symbol_grid_size_mm_pref

Read the default symbol-editor grid size (mm). Falls back to 1.27 mm.

## Signature

```rust
pub fn read_symbol_grid_size_mm_pref() -> f32
```

## Visibility

- `pub`

## Docstring

Read the default symbol-editor grid size (mm). Falls back to 1.27 mm.

## Source
Lines 774–778 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_prefs_json](/crates/oxide-app/src/fonts/mod/read_prefs_json.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
| called_by | [default](/crates/oxide-app/src/library/state/methods/default.md) |

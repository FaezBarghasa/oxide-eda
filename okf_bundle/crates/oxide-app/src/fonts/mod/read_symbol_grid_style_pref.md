---
okf_version: "0.2"
type: Function
title: read_symbol_grid_style_pref
description: "Read the symbol-editor grid style preference. Defaults to `Dots`."
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_symbol_grid_style_pref
language: rust
---

# read_symbol_grid_style_pref

Read the symbol-editor grid style preference. Defaults to `Dots`.

## Signature

```rust
pub fn read_symbol_grid_style_pref() -> GridStyle
```

## Visibility

- `pub`

## Docstring

Read the symbol-editor grid style preference. Defaults to `Dots`.

## Source
Lines 790–804 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_prefs_json](/crates/oxide-app/src/fonts/mod/read_prefs_json.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |

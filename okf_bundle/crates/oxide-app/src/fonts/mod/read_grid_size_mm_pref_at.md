---
okf_version: "0.2"
type: Function
title: read_grid_size_mm_pref_at
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_grid_size_mm_pref_at
language: rust
---

# read_grid_size_mm_pref_at

## Signature

```rust
pub fn read_grid_size_mm_pref_at(path: &Path) -> Option<f32>
```

## Visibility

- `pub`

## Source
Lines 759–761 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_prefs_json](/crates/oxide-app/src/fonts/mod/read_prefs_json.md) |
| called_by | [read_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/read_grid_size_mm_pref.md) |
| called_by | [prefs_grid_size_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_grid_size_round_trip_through_json.md) |

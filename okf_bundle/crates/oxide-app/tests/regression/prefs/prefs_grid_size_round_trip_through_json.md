---
okf_version: "0.2"
type: Function
title: prefs_grid_size_round_trip_through_json
description: "[test]"
resource: crates/oxide-app/tests/regression/prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/prefs/prefs_grid_size_round_trip_through_json
language: rust
---

# prefs_grid_size_round_trip_through_json

[test]

## Signature

```rust
fn prefs_grid_size_round_trip_through_json()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 267–280 in `crates/oxide-app/tests/regression/prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs](/crates/oxide-app/tests/regression/prefs.md) |
| calls | [temp_prefs_path](/crates/oxide-app/tests/regression/prefs/temp_prefs_path.md) |
| calls | [write_grid_size_mm_pref_at](/crates/oxide-app/src/fonts/mod/write_grid_size_mm_pref_at.md) |
| calls | [read_grid_size_mm_pref_at](/crates/oxide-app/src/fonts/mod/read_grid_size_mm_pref_at.md) |

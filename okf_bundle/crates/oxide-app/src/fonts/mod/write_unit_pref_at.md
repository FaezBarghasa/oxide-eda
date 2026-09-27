---
okf_version: "0.2"
type: Function
title: write_unit_pref_at
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/write_unit_pref_at
language: rust
---

# write_unit_pref_at

## Signature

```rust
pub fn write_unit_pref_at(path: &Path, unit: Unit)
```

## Visibility

- `pub`

## Source
Lines 660–666 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| called_by | [write_unit_pref](/crates/oxide-app/src/fonts/mod/write_unit_pref.md) |
| called_by | [prefs_cross_pref_independence](/crates/oxide-app/tests/regression/prefs/prefs_cross_pref_independence.md) |
| called_by | [prefs_unit_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_unit_round_trip_through_json.md) |
| called_by | [prefs_writes_dont_clobber_neighboring_keys](/crates/oxide-app/tests/regression/prefs/prefs_writes_dont_clobber_neighboring_keys.md) |

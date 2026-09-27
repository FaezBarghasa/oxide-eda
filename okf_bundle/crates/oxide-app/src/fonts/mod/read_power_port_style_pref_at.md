---
okf_version: "0.2"
type: Function
title: read_power_port_style_pref_at
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_power_port_style_pref_at
language: rust
---

# read_power_port_style_pref_at

## Signature

```rust
pub fn read_power_port_style_pref_at(path: &Path) -> PowerPortStyle
```

## Visibility

- `pub`

## Source
Lines 446–455 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_prefs_json](/crates/oxide-app/src/fonts/mod/read_prefs_json.md) |
| called_by | [read_power_port_style_pref](/crates/oxide-app/src/fonts/mod/read_power_port_style_pref.md) |

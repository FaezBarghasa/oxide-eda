---
okf_version: "0.2"
type: Function
title: read_power_port_style_pref
description: "Read `power_port_style` from preferences file."
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_power_port_style_pref
language: rust
---

# read_power_port_style_pref

Read `power_port_style` from preferences file.

## Signature

```rust
pub fn read_power_port_style_pref() -> PowerPortStyle
```

## Visibility

- `pub`

## Docstring

Read `power_port_style` from preferences file.
Defaults to `Altium` when missing or invalid.

## Source
Lines 442–444 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_power_port_style_pref_at](/crates/oxide-app/src/fonts/mod/read_power_port_style_pref_at.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |

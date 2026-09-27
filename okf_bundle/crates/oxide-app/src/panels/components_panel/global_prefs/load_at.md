---
okf_version: "0.2"
type: Function
title: load_at
description: Load from a specific path — extracted so tests can hit the actual
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/load_at
language: rust
---

# load_at

Load from a specific path — extracted so tests can hit the actual

## Signature

```rust
pub fn load_at(path: &Path) -> Vec<GlobalLibraryEntry>
```

## Visibility

- `pub`

## Docstring

Load from a specific path — extracted so tests can hit the actual
parse path without going through `prefs_path()`/`config_root()`.

## Source
Lines 67–94 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
| called_by | [load](/crates/oxide-app/src/panels/components_panel/global_prefs/load.md) |

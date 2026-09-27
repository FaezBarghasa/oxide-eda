---
okf_version: "0.2"
type: Function
title: prefs_path
description: "Resolved on-disk path of `global_libraries.toml`. `None` when the"
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/prefs_path
language: rust
---

# prefs_path

Resolved on-disk path of `global_libraries.toml`. `None` when the

## Signature

```rust
pub fn prefs_path() -> Option<PathBuf>
```

## Visibility

- `pub`

## Docstring

Resolved on-disk path of `global_libraries.toml`. `None` when the
platform can't resolve a config dir (very rare — a stripped-down
headless environment). See [`crate::config_root::config_root`].

## Source
Lines 49–52 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
| calls | [config_root](/crates/oxide-app/src/config_root/config_root.md) |
| called_by | [load](/crates/oxide-app/src/panels/components_panel/global_prefs/load.md) |
| called_by | [save](/crates/oxide-app/src/panels/components_panel/global_prefs/save.md) |

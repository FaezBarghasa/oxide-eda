---
okf_version: "0.2"
type: Function
title: save
description: "Persist `entries` to `global_libraries.toml`. Creates the parent"
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/save
language: rust
---

# save

Persist `entries` to `global_libraries.toml`. Creates the parent

## Signature

```rust
pub fn save(entries: &[GlobalLibraryEntry]) -> Result<(), String>
```

## Visibility

- `pub`

## Docstring

Persist `entries` to `global_libraries.toml`. Creates the parent
directory if missing. Errors warn through `tracing` and surface to
the caller via the `Result` so the dispatcher can show a brief
inline error in the Components Panel.

## Source
Lines 100–103 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
| calls | [prefs_path](/crates/oxide-app/src/panels/components_panel/global_prefs/prefs_path.md) |
| calls | [save_at](/crates/oxide-app/src/panels/components_panel/global_prefs/save_at.md) |
| called_by | [add_path](/crates/oxide-app/src/panels/components_panel/global_prefs/add_path.md) |
| called_by | [remove_path](/crates/oxide-app/src/panels/components_panel/global_prefs/remove_path.md) |

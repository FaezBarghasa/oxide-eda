---
okf_version: "0.2"
type: Function
title: remove_path
description: Remove a path from the global list and persist.
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/remove_path
language: rust
---

# remove_path

Remove a path from the global list and persist.

## Signature

```rust
pub fn remove_path(path: &Path) -> Result<Vec<GlobalLibraryEntry>, String>
```

## Visibility

- `pub`

## Docstring

Remove a path from the global list and persist.

## Source
Lines 141–149 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
| calls | [load](/crates/oxide-app/src/panels/components_panel/global_prefs/load.md) |
| calls | [save](/crates/oxide-app/src/panels/components_panel/global_prefs/save.md) |

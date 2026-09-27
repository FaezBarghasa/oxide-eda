---
okf_version: "0.2"
type: Function
title: synthetic_tab_path
description: Synthetic on-disk identity for a Component Preview tab — used by
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/synthetic_tab_path_1
language: rust
---

# synthetic_tab_path

Synthetic on-disk identity for a Component Preview tab — used by

## Signature

```rust
pub fn synthetic_tab_path(&self) -> PathBuf
```

## Visibility

- `pub`

## Docstring

Synthetic on-disk identity for a Component Preview tab — used by
`TabInfo.path` so the tab bar, undock detector, and dirty-paths
machinery have a single unique `PathBuf` per row without needing
a second identity scheme. The path points at the row's home table
with the `row_id` as a suffix so the synthetic key is unique
per-row even when multiple rows share a table.

## Source
Lines 69–73 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |

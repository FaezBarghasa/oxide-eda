---
okf_version: "0.2"
type: Function
title: read_grid_visible_pref
description: "Read the last grid-visible toggle. Defaults to `true`."
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_grid_visible_pref
language: rust
---

# read_grid_visible_pref

Read the last grid-visible toggle. Defaults to `true`.

## Signature

```rust
pub fn read_grid_visible_pref() -> bool
```

## Visibility

- `pub`

## Docstring

Read the last grid-visible toggle. Defaults to `true`.

## Source
Lines 673–675 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_grid_visible_pref_at](/crates/oxide-app/src/fonts/mod/read_grid_visible_pref_at.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |

---
okf_version: "0.2"
type: Function
title: read_unit_pref
description: "Read the last-active coordinate unit. Defaults to `Unit::Mm`."
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_unit_pref
language: rust
---

# read_unit_pref

Read the last-active coordinate unit. Defaults to `Unit::Mm`.

## Signature

```rust
pub fn read_unit_pref() -> Unit
```

## Visibility

- `pub`

## Docstring

Read the last-active coordinate unit. Defaults to `Unit::Mm`.

## Source
Lines 644–646 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_unit_pref_at](/crates/oxide-app/src/fonts/mod/read_unit_pref_at.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |

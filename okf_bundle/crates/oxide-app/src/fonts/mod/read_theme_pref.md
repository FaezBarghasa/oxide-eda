---
okf_version: "0.2"
type: Function
title: read_theme_pref
description: "Read the last-applied theme. Defaults to `ThemeId::Oxide`."
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_theme_pref
language: rust
---

# read_theme_pref

Read the last-applied theme. Defaults to `ThemeId::Oxide`.

## Signature

```rust
pub fn read_theme_pref() -> ThemeId
```

## Visibility

- `pub`

## Docstring

Read the last-applied theme. Defaults to `ThemeId::Oxide`.

## Source
Lines 612–614 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_theme_pref_at](/crates/oxide-app/src/fonts/mod/read_theme_pref_at.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |

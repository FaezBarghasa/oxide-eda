---
okf_version: "0.2"
type: Function
title: read_theme_pref_at
description: "Same as [`read_theme_pref`] but reads from `path` — exposed for"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_theme_pref_at
language: rust
---

# read_theme_pref_at

Same as [`read_theme_pref`] but reads from `path` — exposed for

## Signature

```rust
pub fn read_theme_pref_at(path: &Path) -> ThemeId
```

## Visibility

- `pub`

## Docstring

Same as [`read_theme_pref`] but reads from `path` — exposed for
integration tests that inject a tempdir prefs file.

## Source
Lines 618–623 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_prefs_json](/crates/oxide-app/src/fonts/mod/read_prefs_json.md) |
| called_by | [read_theme_pref](/crates/oxide-app/src/fonts/mod/read_theme_pref.md) |

---
okf_version: "0.2"
type: Function
title: read_ui_font_pref
description: "Read only the `ui_font` key from the preferences file."
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_ui_font_pref
language: rust
---

# read_ui_font_pref

Read only the `ui_font` key from the preferences file.

## Signature

```rust
pub fn read_ui_font_pref() -> String
```

## Visibility

- `pub`

## Docstring

Read only the `ui_font` key from the preferences file.
Returns `DEFAULT_UI_FONT` if the file is absent, malformed, or the key missing.

## Source
Lines 379–381 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_ui_font_pref_at](/crates/oxide-app/src/fonts/mod/read_ui_font_pref_at.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
| called_by | [main](/crates/oxide-app/src/main/main.md) |

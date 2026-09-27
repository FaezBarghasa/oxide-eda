---
okf_version: "0.2"
type: Function
title: write_component_classes_pref
description: "Persist `classes` to the `component_classes` array in prefs.json"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/write_component_classes_pref
language: rust
---

# write_component_classes_pref

Persist `classes` to the `component_classes` array in prefs.json

## Signature

```rust
pub fn write_component_classes_pref(classes: &[ComponentClassEntry])
```

## Visibility

- `pub`

## Docstring

Persist `classes` to the `component_classes` array in prefs.json
without clobbering other preference keys. Silent on I/O failure
— preferences are best-effort.

## Source
Lines 432–438 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |

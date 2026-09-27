---
okf_version: "0.2"
type: Function
title: absent_prefs_file_is_created
description: "The fresh-install path: absence is the one state that legitimately"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/absent_prefs_file_is_created
language: rust
---

# absent_prefs_file_is_created

The fresh-install path: absence is the one state that legitimately

## Signature

```rust
fn absent_prefs_file_is_created()
```

## Decorators

- `test`

## Docstring

The fresh-install path: absence is the one state that legitimately
starts from an empty object.
[test]

## Source
Lines 469–486 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| calls | [read_prefs_object](/crates/oxide-app/src/fonts/prefs_file/read_prefs_object.md) |

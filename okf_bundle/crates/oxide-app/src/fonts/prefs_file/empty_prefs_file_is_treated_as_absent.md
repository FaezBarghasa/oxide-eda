---
okf_version: "0.2"
type: Function
title: empty_prefs_file_is_treated_as_absent
description: A zero-byte file carries no user data to protect and is what an
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/empty_prefs_file_is_treated_as_absent
language: rust
---

# empty_prefs_file_is_treated_as_absent

A zero-byte file carries no user data to protect and is what an

## Signature

```rust
fn empty_prefs_file_is_treated_as_absent()
```

## Decorators

- `test`

## Docstring

A zero-byte file carries no user data to protect and is what an
interrupted writer leaves behind, so it is treated as absent.
[test]

## Source
Lines 491–504 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| calls | [read_prefs_object](/crates/oxide-app/src/fonts/prefs_file/read_prefs_object.md) |

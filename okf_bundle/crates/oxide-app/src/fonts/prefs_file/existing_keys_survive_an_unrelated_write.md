---
okf_version: "0.2"
type: Function
title: existing_keys_survive_an_unrelated_write
description: "The base property the whole module exists for: writing one key"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/existing_keys_survive_an_unrelated_write
language: rust
---

# existing_keys_survive_an_unrelated_write

The base property the whole module exists for: writing one key

## Signature

```rust
fn existing_keys_survive_an_unrelated_write()
```

## Decorators

- `test`

## Docstring

The base property the whole module exists for: writing one key
leaves every other key untouched.
[test]

## Source
Lines 538–565 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| calls | [read_prefs_object](/crates/oxide-app/src/fonts/prefs_file/read_prefs_object.md) |

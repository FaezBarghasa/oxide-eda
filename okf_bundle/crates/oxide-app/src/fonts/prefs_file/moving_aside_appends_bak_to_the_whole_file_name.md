---
okf_version: "0.2"
type: Function
title: moving_aside_appends_bak_to_the_whole_file_name
description: "`set_extension` would turn `prefs.json` into `prefs.bak` and lose"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/moving_aside_appends_bak_to_the_whole_file_name
language: rust
---

# moving_aside_appends_bak_to_the_whole_file_name

`set_extension` would turn `prefs.json` into `prefs.bak` and lose

## Signature

```rust
fn moving_aside_appends_bak_to_the_whole_file_name()
```

## Decorators

- `test`

## Docstring

`set_extension` would turn `prefs.json` into `prefs.bak` and lose
which file it came from; the suffix goes on the whole file name.
[test]

## Source
Lines 804–831 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |

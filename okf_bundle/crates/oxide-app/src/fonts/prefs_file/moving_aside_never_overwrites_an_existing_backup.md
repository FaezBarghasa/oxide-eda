---
okf_version: "0.2"
type: Function
title: moving_aside_never_overwrites_an_existing_backup
description: "The keymap backup returns `Ok(None)` when a `.bak` exists, which"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/moving_aside_never_overwrites_an_existing_backup
language: rust
---

# moving_aside_never_overwrites_an_existing_backup

The keymap backup returns `Ok(None)` when a `.bak` exists, which

## Signature

```rust
fn moving_aside_never_overwrites_an_existing_backup()
```

## Decorators

- `test`

## Docstring

The keymap backup returns `Ok(None)` when a `.bak` exists, which
is right there — that backup IS the original. Here it would trap
the user on their second reset, so the search ladders instead.
[test]

## Source
Lines 837–865 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |

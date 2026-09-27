---
okf_version: "0.2"
type: Function
title: unreadable_prefs_file_is_left_alone
description: A directory standing where the file should be is the portable
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/unreadable_prefs_file_is_left_alone
language: rust
---

# unreadable_prefs_file_is_left_alone

A directory standing where the file should be is the portable

## Signature

```rust
fn unreadable_prefs_file_is_left_alone()
```

## Decorators

- `test`

## Docstring

A directory standing where the file should be is the portable
stand-in for "readable() fails with something other than
NotFound": `std::fs::read` on a directory fails on every
platform, and — unlike a `chmod 000` file — it still fails when
the suite runs as root.
[test]

## Source
Lines 449–464 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |

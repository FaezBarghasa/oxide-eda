---
okf_version: "0.2"
type: Function
title: malformed_prefs_file_is_left_byte_identical
description: "The #594 regression: a truncated file (killed mid-write, or"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/malformed_prefs_file_is_left_byte_identical
language: rust
---

# malformed_prefs_file_is_left_byte_identical

The #594 regression: a truncated file (killed mid-write, or

## Signature

```rust
fn malformed_prefs_file_is_left_byte_identical()
```

## Decorators

- `test`

## Docstring

The #594 regression: a truncated file (killed mid-write, or
hand-edited with a brace missing) must survive the next write.
[test]

## Source
Lines 420–441 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |

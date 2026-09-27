---
okf_version: "0.2"
type: Function
title: checking_an_empty_file_reports_it_as_healthy
description: "Same rule as [`empty_prefs_file_is_treated_as_absent`]: a writer"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/checking_an_empty_file_reports_it_as_healthy
language: rust
---

# checking_an_empty_file_reports_it_as_healthy

Same rule as [`empty_prefs_file_is_treated_as_absent`]: a writer

## Signature

```rust
fn checking_an_empty_file_reports_it_as_healthy()
```

## Decorators

- `test`

## Docstring

Same rule as [`empty_prefs_file_is_treated_as_absent`]: a writer
killed between `File::create` and `write_all` leaves a zero-byte
file that carries no user data, and the next write recreates it.
[test]

## Source
Lines 591–605 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [check_prefs_file_at](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file_at.md) |

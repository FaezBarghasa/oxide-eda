---
okf_version: "0.2"
type: Function
title: load_reports_error_when_the_file_cannot_be_parsed
description: A shortcuts file that exists but cannot be parsed must surface as
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/load_reports_error_when_the_file_cannot_be_parsed
language: rust
---

# load_reports_error_when_the_file_cannot_be_parsed

A shortcuts file that exists but cannot be parsed must surface as

## Signature

```rust
fn load_reports_error_when_the_file_cannot_be_parsed()
```

## Decorators

- `test`

## Docstring

A shortcuts file that exists but cannot be parsed must surface as
`Err`, not as a silent fall back to the built-ins — that `Err` is what
raises the Preferences banner and arms the backup-before-save guard.
[test]

## Source
Lines 242–253 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [load_profile_set_at](/crates/oxide-app/src/keymap/profile/load_profile_set_at.md) |

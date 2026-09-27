---
okf_version: "0.2"
type: Function
title: checking_a_file_never_creates_or_modifies_it
description: "The probe runs on every Preferences open, so it must be inert: it"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/checking_a_file_never_creates_or_modifies_it
language: rust
---

# checking_a_file_never_creates_or_modifies_it

The probe runs on every Preferences open, so it must be inert: it

## Signature

```rust
fn checking_a_file_never_creates_or_modifies_it()
```

## Decorators

- `test`

## Docstring

The probe runs on every Preferences open, so it must be inert: it
may not create the file it is asked about, and it may not rewrite
one that is already there.
[test]

## Source
Lines 666–689 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [check_prefs_file_at](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file_at.md) |

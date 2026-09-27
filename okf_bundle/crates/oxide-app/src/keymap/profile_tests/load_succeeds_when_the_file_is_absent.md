---
okf_version: "0.2"
type: Function
title: load_succeeds_when_the_file_is_absent
description: "The non-error path must stay non-error: a missing file is a fresh"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/load_succeeds_when_the_file_is_absent
language: rust
---

# load_succeeds_when_the_file_is_absent

The non-error path must stay non-error: a missing file is a fresh

## Signature

```rust
fn load_succeeds_when_the_file_is_absent()
```

## Decorators

- `test`

## Docstring

The non-error path must stay non-error: a missing file is a fresh
install, not a failure, and must never raise the banner.
[test]

## Source
Lines 287–295 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [load_profile_set_at](/crates/oxide-app/src/keymap/profile/load_profile_set_at.md) |

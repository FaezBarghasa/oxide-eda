---
okf_version: "0.2"
type: Function
title: load_reports_error_when_the_active_profile_id_does_not_resolve
description: "A well-formed file whose `active_profile` no longer resolves fails in"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/load_reports_error_when_the_active_profile_id_does_not_resolve
language: rust
---

# load_reports_error_when_the_active_profile_id_does_not_resolve

A well-formed file whose `active_profile` no longer resolves fails in

## Signature

```rust
fn load_reports_error_when_the_active_profile_id_does_not_resolve()
```

## Decorators

- `test`

## Docstring

A well-formed file whose `active_profile` no longer resolves fails in
`apply_to` -> `set_active_profile`. The user's custom profiles are all
still in that file, so this must not be mistaken for "no shortcuts".
[test]

## Source
Lines 259–282 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
| calls | [load_profile_set_at](/crates/oxide-app/src/keymap/profile/load_profile_set_at.md) |

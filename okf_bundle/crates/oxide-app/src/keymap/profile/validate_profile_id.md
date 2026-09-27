---
okf_version: "0.2"
type: Function
title: validate_profile_id
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/validate_profile_id
language: rust
---

# validate_profile_id

## Signature

```rust
fn validate_profile_id(id: &str) -> Result<(), ProfileLoadError>
```

## Source
Lines 856–865 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| called_by | [copy_as_custom](/crates/oxide-app/src/keymap/profile/copy_as_custom.md) |
| called_by | [from_profile](/crates/oxide-app/src/keymap/profile/from_profile.md) |
| called_by | [insert_custom_profile](/crates/oxide-app/src/keymap/profile/insert_custom_profile.md) |
| called_by | [into_profile](/crates/oxide-app/src/keymap/profile/into_profile.md) |
| called_by | [new](/crates/oxide-app/src/keymap/profile/new.md) |

---
okf_version: "0.2"
type: Function
title: copy_as_custom
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/copy_as_custom_1
language: rust
---

# copy_as_custom

## Signature

```rust
pub fn copy_as_custom(
        &self,
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, ProfileLoadError>
```

## Visibility

- `pub`

## Source
Lines 37–53 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [validate_profile_id](/crates/oxide-app/src/keymap/profile/validate_profile_id.md) |

---
okf_version: "0.2"
type: Function
title: set_secret
description: Persist the secret. Overwrites any existing value.
resource: crates/oxide-library/src/distributors/keyring.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:13:21Z"
concept_id: crates/oxide-library/src/distributors/keyring/set_secret
language: rust
---

# set_secret

Persist the secret. Overwrites any existing value.

## Signature

```rust
impl KeyringStore { pub fn set_secret(&self, secret: &str) -> Result<(), KeyringError> }
```

## Visibility

- `pub`

## Docstring

Persist the secret. Overwrites any existing value.

## Source
Lines 82–84 in `crates/oxide-library/src/distributors/keyring.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keyring](/crates/oxide-library/src/distributors/keyring.md) |

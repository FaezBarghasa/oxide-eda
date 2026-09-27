---
okf_version: "0.2"
type: Function
title: get_secret
description: "Read the stored secret. Returns `KeyringError::NotFound` if absent."
resource: crates/oxide-library/src/distributors/keyring.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:13:21Z"
concept_id: crates/oxide-library/src/distributors/keyring/get_secret
language: rust
---

# get_secret

Read the stored secret. Returns `KeyringError::NotFound` if absent.

## Signature

```rust
impl KeyringStore { pub fn get_secret(&self) -> Result<String, KeyringError> }
```

## Visibility

- `pub`

## Docstring

Read the stored secret. Returns `KeyringError::NotFound` if absent.

## Source
Lines 87–89 in `crates/oxide-library/src/distributors/keyring.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keyring](/crates/oxide-library/src/distributors/keyring.md) |

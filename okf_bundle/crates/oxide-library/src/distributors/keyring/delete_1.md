---
okf_version: "0.2"
type: Function
title: delete
description: "Delete the entry. Idempotent: deleting an absent entry is `Ok`."
resource: crates/oxide-library/src/distributors/keyring.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:13:21Z"
concept_id: crates/oxide-library/src/distributors/keyring/delete_1
language: rust
---

# delete

Delete the entry. Idempotent: deleting an absent entry is `Ok`.

## Signature

```rust
pub fn delete(&self) -> Result<(), KeyringError>
```

## Visibility

- `pub`

## Docstring

Delete the entry. Idempotent: deleting an absent entry is `Ok`.

## Source
Lines 92–98 in `crates/oxide-library/src/distributors/keyring.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keyring](/crates/oxide-library/src/distributors/keyring.md) |

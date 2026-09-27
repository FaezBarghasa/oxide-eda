---
okf_version: "0.2"
type: Function
title: service_name
description: Service name as registered with the OS keychain.
resource: crates/oxide-library/src/distributors/keyring.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:13:21Z"
concept_id: crates/oxide-library/src/distributors/keyring/service_name
language: rust
---

# service_name

Service name as registered with the OS keychain.

## Signature

```rust
impl KeyringStore { pub fn service_name(&self) -> &str }
```

## Visibility

- `pub`

## Docstring

Service name as registered with the OS keychain.

## Source
Lines 72–74 in `crates/oxide-library/src/distributors/keyring.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keyring](/crates/oxide-library/src/distributors/keyring.md) |

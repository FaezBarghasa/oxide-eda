---
okf_version: "0.2"
type: Class
title: KeyringStore
description: "Wrapper around a single keyring entry, scoped to one distributor provider."
resource: crates/oxide-library/src/distributors/keyring.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:13:21Z"
concept_id: crates/oxide-library/src/distributors/keyring/KeyringStore
language: rust
---

# KeyringStore

Wrapper around a single keyring entry, scoped to one distributor provider.

## Signature

```rust
pub struct KeyringStore
```

## Visibility

- `pub`

## Docstring

Wrapper around a single keyring entry, scoped to one distributor provider.

One `KeyringStore` instance maps to one underlying OS keychain item. The
service name follows the spec: `oxide-distributor-<provider>` (e.g.
`oxide-distributor-digikey`). The username slot lets callers separate
e.g. an OAuth access token from a refresh token (`"access"`/`"refresh"`).

## Methods

- `service`
- `username`
- `entry`

## Source
Lines 36–40 in `crates/oxide-library/src/distributors/keyring.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keyring](/crates/oxide-library/src/distributors/keyring.md) |

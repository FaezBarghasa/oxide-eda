---
okf_version: "0.2"
type: Function
title: keyring_available
description: "Helper: did the keyring backend even initialise? Many CI envs (Linux"
resource: crates/oxide-library/tests/distributor_keyring.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/distributor_keyring/keyring_available
language: rust
---

# keyring_available

Helper: did the keyring backend even initialise? Many CI envs (Linux

## Signature

```rust
fn keyring_available(store: &KeyringStore) -> bool
```

## Docstring

Helper: did the keyring backend even initialise? Many CI envs (Linux
without Secret Service, macOS sandboxed runners) return `Backend(_)` on
first set/get; we detect that and skip without failing the test.

## Source
Lines 28–37 in `crates/oxide-library/tests/distributor_keyring.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_keyring](/crates/oxide-library/tests/distributor_keyring.md) |
| called_by | [keyring_delete_is_idempotent](/crates/oxide-library/tests/distributor_keyring/keyring_delete_is_idempotent.md) |
| called_by | [keyring_missing_entry_yields_not_found](/crates/oxide-library/tests/distributor_keyring/keyring_missing_entry_yields_not_found.md) |
| called_by | [keyring_overwrite_replaces_value](/crates/oxide-library/tests/distributor_keyring/keyring_overwrite_replaces_value.md) |
| called_by | [keyring_round_trip](/crates/oxide-library/tests/distributor_keyring/keyring_round_trip.md) |

---
okf_version: "0.2"
type: Function
title: entry_for
resource: crates/oxide-library/tests/distributor_keyring.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/distributor_keyring/entry_for
language: rust
---

# entry_for

## Signature

```rust
fn entry_for(test_name: &str) -> KeyringStore
```

## Source
Lines 12–19 in `crates/oxide-library/tests/distributor_keyring.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_keyring](/crates/oxide-library/tests/distributor_keyring.md) |
| called_by | [keyring_delete_is_idempotent](/crates/oxide-library/tests/distributor_keyring/keyring_delete_is_idempotent.md) |
| called_by | [keyring_missing_entry_yields_not_found](/crates/oxide-library/tests/distributor_keyring/keyring_missing_entry_yields_not_found.md) |
| called_by | [keyring_overwrite_replaces_value](/crates/oxide-library/tests/distributor_keyring/keyring_overwrite_replaces_value.md) |
| called_by | [keyring_round_trip](/crates/oxide-library/tests/distributor_keyring/keyring_round_trip.md) |

---
okf_version: "0.2"
type: Function
title: cleanup
resource: crates/oxide-library/tests/distributor_keyring.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/distributor_keyring/cleanup
language: rust
---

# cleanup

## Signature

```rust
fn cleanup(store: &KeyringStore)
```

## Source
Lines 21–23 in `crates/oxide-library/tests/distributor_keyring.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_keyring](/crates/oxide-library/tests/distributor_keyring.md) |
| called_by | [enable_project_version_control](/crates/oxide-library/src/lib/enable_project_version_control.md) |
| called_by | [keyring_overwrite_replaces_value](/crates/oxide-library/tests/distributor_keyring/keyring_overwrite_replaces_value.md) |
| called_by | [keyring_round_trip](/crates/oxide-library/tests/distributor_keyring/keyring_round_trip.md) |

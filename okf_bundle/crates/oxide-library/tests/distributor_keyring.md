---
okf_version: "0.2"
type: Module
title: distributor_keyring
description: "Integration tests for `KeyringStore`."
resource: crates/oxide-library/tests/distributor_keyring.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/distributor_keyring
language: rust
---

# distributor_keyring

Integration tests for `KeyringStore`.

## Docstring

Integration tests for `KeyringStore`.

Round-trip test gated by platform availability. Windows
Credential Manager is available in tests; Linux/macOS CI runners
often lack a `Secret Service` daemon, so those tests run
conditionally.

## Relationships

| Type | Target |
|------|--------|
| related | [entry_for](/crates/oxide-library/tests/distributor_keyring/entry_for.md) |
| related | [cleanup](/crates/oxide-library/tests/distributor_keyring/cleanup.md) |
| related | [keyring_available](/crates/oxide-library/tests/distributor_keyring/keyring_available.md) |
| related | [keyring_round_trip](/crates/oxide-library/tests/distributor_keyring/keyring_round_trip.md) |
| related | [keyring_missing_entry_yields_not_found](/crates/oxide-library/tests/distributor_keyring/keyring_missing_entry_yields_not_found.md) |
| related | [keyring_delete_is_idempotent](/crates/oxide-library/tests/distributor_keyring/keyring_delete_is_idempotent.md) |
| related | [keyring_overwrite_replaces_value](/crates/oxide-library/tests/distributor_keyring/keyring_overwrite_replaces_value.md) |
| related | [service_name_format_matches_spec](/crates/oxide-library/tests/distributor_keyring/service_name_format_matches_spec.md) |

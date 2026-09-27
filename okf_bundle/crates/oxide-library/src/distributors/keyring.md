---
okf_version: "0.2"
type: Module
title: keyring
description: OS keyring credential storage for distributor adapters.
resource: crates/oxide-library/src/distributors/keyring.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:13:21Z"
concept_id: crates/oxide-library/src/distributors/keyring
language: rust
---

# keyring

OS keyring credential storage for distributor adapters.

## Docstring

OS keyring credential storage for distributor adapters.

- Service name format: `oxide-distributor-<provider>`
- Used by Mouser (API key) and DigiKey (OAuth refresh token).
- Tests gated by platform: Windows Credential Manager works; Linux/macOS
CI runners may lack a backend → callers must handle
`KeyringError::Backend` gracefully.

## Relationships

| Type | Target |
|------|--------|
| related | [KeyringError](/crates/oxide-library/src/distributors/keyring/KeyringError.md) |
| related | [from](/crates/oxide-library/src/distributors/keyring/from.md) |
| related | [from](/crates/oxide-library/src/distributors/keyring/from.md) |
| related | [KeyringStore](/crates/oxide-library/src/distributors/keyring/KeyringStore.md) |
| related | [fmt](/crates/oxide-library/src/distributors/keyring/fmt.md) |
| related | [fmt](/crates/oxide-library/src/distributors/keyring/fmt.md) |
| related | [for_provider](/crates/oxide-library/src/distributors/keyring/for_provider.md) |
| related | [service_name](/crates/oxide-library/src/distributors/keyring/service_name.md) |
| related | [username](/crates/oxide-library/src/distributors/keyring/username.md) |
| related | [set_secret](/crates/oxide-library/src/distributors/keyring/set_secret.md) |
| related | [get_secret](/crates/oxide-library/src/distributors/keyring/get_secret.md) |
| related | [delete](/crates/oxide-library/src/distributors/keyring/delete.md) |
| related | [for_provider](/crates/oxide-library/src/distributors/keyring/for_provider.md) |
| related | [service_name](/crates/oxide-library/src/distributors/keyring/service_name.md) |
| related | [username](/crates/oxide-library/src/distributors/keyring/username.md) |
| related | [set_secret](/crates/oxide-library/src/distributors/keyring/set_secret.md) |
| related | [get_secret](/crates/oxide-library/src/distributors/keyring/get_secret.md) |
| related | [delete](/crates/oxide-library/src/distributors/keyring/delete.md) |
| related | [service_prefix_is_stable](/crates/oxide-library/src/distributors/keyring/service_prefix_is_stable.md) |
| related | [for_provider_builds_expected_service_name](/crates/oxide-library/src/distributors/keyring/for_provider_builds_expected_service_name.md) |

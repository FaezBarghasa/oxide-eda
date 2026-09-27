---
okf_version: "0.2"
type: Function
title: from_keyring
description: "Production constructor: pulls the API key from `oxide-distributor-mouser`"
resource: crates/oxide-library/src/distributors/mouser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/mouser/from_keyring_1
language: rust
---

# from_keyring

Production constructor: pulls the API key from `oxide-distributor-mouser`

## Signature

```rust
pub fn from_keyring(
        cache: Option<DistributorCache>,
    ) -> Result<Self, super::keyring::KeyringError>
```

## Visibility

- `pub`

## Docstring

Production constructor: pulls the API key from `oxide-distributor-mouser`
at request time. The username slot defaults to `"default"` to match
what the eventual UI will write.

MD-17: returns `Result` because the OS keychain may not be
available (Linux Docker without dbus, etc). On error, callers
can fall back to `with_api_key` (env-var-driven) or surface a
"keychain unavailable" message to the user.

## Source
Lines 57–70 in `crates/oxide-library/src/distributors/mouser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mouser](/crates/oxide-library/src/distributors/mouser.md) |

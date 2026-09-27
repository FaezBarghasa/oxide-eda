---
okf_version: "0.2"
type: Function
title: for_provider
description: "Create a store for `provider` with the given `username` slot."
resource: crates/oxide-library/src/distributors/keyring.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:13:21Z"
concept_id: crates/oxide-library/src/distributors/keyring/for_provider
language: rust
---

# for_provider

Create a store for `provider` with the given `username` slot.

## Signature

```rust
impl KeyringStore { pub fn for_provider(provider: &str, username: &str) -> Result<Self, KeyringError> }
```

## Visibility

- `pub`

## Docstring

Create a store for `provider` with the given `username` slot.

MD-17: returns `Result` because `keyring::Entry::new` can fail on
platforms without a daemon (Linux Docker without dbus / libsecret,
minimal Wayland setups, headless CI). The previous `expect()`
panicked the calling thread — typically the iced UI thread on
app startup — which is unrecoverable. Callers now propagate the
error to the user (e.g. "Distributor unavailable: install
libsecret-tools or run with `--no-keyring`").

## Source
Lines 61–69 in `crates/oxide-library/src/distributors/keyring.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keyring](/crates/oxide-library/src/distributors/keyring.md) |

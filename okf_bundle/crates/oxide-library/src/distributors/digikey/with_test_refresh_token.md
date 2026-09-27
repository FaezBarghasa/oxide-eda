---
okf_version: "0.2"
type: Function
title: with_test_refresh_token
description: "Test-only setter: provide an in-memory refresh token. When set,"
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/with_test_refresh_token
language: rust
---

# with_test_refresh_token

Test-only setter: provide an in-memory refresh token. When set,

## Signature

```rust
impl DigiKeyAuth { pub fn with_test_refresh_token(mut self, refresh: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Docstring

Test-only setter: provide an in-memory refresh token. When set,
[`Self::access_token`] uses this instead of the keyring.
[doc(hidden)]

## Source
Lines 148–151 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |

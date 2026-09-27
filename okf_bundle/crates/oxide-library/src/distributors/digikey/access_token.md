---
okf_version: "0.2"
type: Function
title: access_token
description: "Step 3 (every API call): use the keyring-stored refresh token to"
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/access_token
language: rust
---

# access_token

Step 3 (every API call): use the keyring-stored refresh token to

## Signature

```rust
impl DigiKeyAuth { pub fn access_token(&self) -> Result<String, DigiKeyAuthError> }
```

## Visibility

- `pub`

## Docstring

Step 3 (every API call): use the keyring-stored refresh token to
mint a fresh access token. Honours [`Self::with_test_refresh_token`]
when set (tests).

## Source
Lines 214–249 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |
| calls | [build_oauth_client](/crates/oxide-library/src/distributors/digikey/build_oauth_client.md) |

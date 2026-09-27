---
okf_version: "0.2"
type: Function
title: start_authorization
description: "Step 1 of the flow: produce the authorization URL the UI should open"
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/start_authorization
language: rust
---

# start_authorization

Step 1 of the flow: produce the authorization URL the UI should open

## Signature

```rust
impl DigiKeyAuth { pub fn start_authorization(&self) -> (Url, CsrfToken, PkceCodeVerifier) }
```

## Visibility

- `pub`

## Docstring

Step 1 of the flow: produce the authorization URL the UI should open
in a browser. Returned tuple: `(url, csrf_token, pkce_verifier)`.
The UI keeps the verifier until the redirect callback fires.

## Source
Lines 156–165 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |

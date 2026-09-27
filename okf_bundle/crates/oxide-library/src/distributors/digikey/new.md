---
okf_version: "0.2"
type: Function
title: new
description: "Production constructor: real DigiKey endpoints, refresh token in"
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/new
language: rust
---

# new

Production constructor: real DigiKey endpoints, refresh token in

## Signature

```rust
impl DigiKeyAuth { pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Result<Self, DigiKeyAuthError> }
```

## Visibility

- `pub`

## Docstring

Production constructor: real DigiKey endpoints, refresh token in
`oxide-distributor-digikey/refresh`.

## Source
Lines 97–109 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |

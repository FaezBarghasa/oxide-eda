---
okf_version: "0.2"
type: Function
title: new
description: Production constructor.
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/new_2
language: rust
---

# new

Production constructor.

## Signature

```rust
impl DigiKeyAdapter { pub fn new(auth: DigiKeyAuth, cache: Option<DistributorCache>) -> Self }
```

## Visibility

- `pub`

## Docstring

Production constructor.

## Source
Lines 291–299 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |
| calls | [build_http_client](/crates/oxide-library/src/distributors/digikey/build_http_client.md) |

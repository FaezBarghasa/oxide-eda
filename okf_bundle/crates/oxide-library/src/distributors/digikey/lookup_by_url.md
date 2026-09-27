---
okf_version: "0.2"
type: Function
title: lookup_by_url
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/lookup_by_url
language: rust
---

# lookup_by_url

## Signature

```rust
impl DigiKeyAdapter { fn lookup_by_url(&self, url: &Url) -> Result<Option<DistributorPart>, DistributorError> }
```

## Source
Lines 410–433 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |

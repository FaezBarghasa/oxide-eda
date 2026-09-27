---
okf_version: "0.2"
type: Function
title: search_by_keyword
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/search_by_keyword
language: rust
---

# search_by_keyword

## Signature

```rust
impl DigiKeyAdapter { fn search_by_keyword(&self, mpn: &str) -> Result<Vec<DistributorPart>, DistributorError> }
```

## Source
Lines 355–398 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |
| calls | [Network](/crates/oxide-widgets/src/passive_calculator/network/Network.md) |

---
okf_version: "0.2"
type: Function
title: search_by_keyword
resource: crates/oxide-library/src/distributors/mouser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/mouser/search_by_keyword
language: rust
---

# search_by_keyword

## Signature

```rust
impl MouserAdapter { fn search_by_keyword(&self, mpn: &str) -> Result<Vec<DistributorPart>, DistributorError> }
```

## Source
Lines 115–168 in `crates/oxide-library/src/distributors/mouser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mouser](/crates/oxide-library/src/distributors/mouser.md) |
| calls | [Network](/crates/oxide-widgets/src/passive_calculator/network/Network.md) |

---
okf_version: "0.2"
type: Function
title: search_by_keyword
resource: crates/oxide-library/src/distributors/jlcpcb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/jlcpcb/search_by_keyword
language: rust
---

# search_by_keyword

## Signature

```rust
impl JlcpcbAdapter { fn search_by_keyword(&self, mpn: &str) -> Result<Vec<DistributorPart>, DistributorError> }
```

## Source
Lines 105–113 in `crates/oxide-library/src/distributors/jlcpcb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [jlcpcb](/crates/oxide-library/src/distributors/jlcpcb.md) |

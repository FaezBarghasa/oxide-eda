---
okf_version: "0.2"
type: Function
title: lookup_by_mpn
resource: crates/oxide-library/src/distributors/jlcpcb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/jlcpcb/lookup_by_mpn
language: rust
---

# lookup_by_mpn

## Signature

```rust
impl JlcpcbAdapter { fn lookup_by_mpn(&self, mpn: &str) -> Result<Vec<DistributorPart>, DistributorError> }
```

## Source
Lines 151–169 in `crates/oxide-library/src/distributors/jlcpcb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [jlcpcb](/crates/oxide-library/src/distributors/jlcpcb.md) |

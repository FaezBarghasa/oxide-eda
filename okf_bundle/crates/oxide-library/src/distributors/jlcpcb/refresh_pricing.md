---
okf_version: "0.2"
type: Function
title: refresh_pricing
resource: crates/oxide-library/src/distributors/jlcpcb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/jlcpcb/refresh_pricing
language: rust
---

# refresh_pricing

## Signature

```rust
impl JlcpcbAdapter { fn refresh_pricing(
        &self,
        part: &DistributorPart,
    ) -> Result<crate::distributor::PricingSnapshot, DistributorError> }
```

## Source
Lines 171–178 in `crates/oxide-library/src/distributors/jlcpcb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [jlcpcb](/crates/oxide-library/src/distributors/jlcpcb.md) |

---
okf_version: "0.2"
type: Function
title: lookup_by_mpn
resource: crates/oxide-library/src/distributors/lcsc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/lcsc/lookup_by_mpn
language: rust
---

# lookup_by_mpn

## Signature

```rust
impl LcscAdapter { fn lookup_by_mpn(&self, mpn: &str) -> Result<Vec<DistributorPart>, DistributorError> }
```

## Source
Lines 178–196 in `crates/oxide-library/src/distributors/lcsc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lcsc](/crates/oxide-library/src/distributors/lcsc.md) |

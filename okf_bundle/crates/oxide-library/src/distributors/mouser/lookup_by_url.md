---
okf_version: "0.2"
type: Function
title: lookup_by_url
resource: crates/oxide-library/src/distributors/mouser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/mouser/lookup_by_url
language: rust
---

# lookup_by_url

## Signature

```rust
impl MouserAdapter { fn lookup_by_url(&self, url: &Url) -> Result<Option<DistributorPart>, DistributorError> }
```

## Source
Lines 180–205 in `crates/oxide-library/src/distributors/mouser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mouser](/crates/oxide-library/src/distributors/mouser.md) |

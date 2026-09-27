---
okf_version: "0.2"
type: Function
title: http_get_json
resource: crates/oxide-library/src/distributors/lcsc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/lcsc/http_get_json_1
language: rust
---

# http_get_json

## Signature

```rust
fn http_get_json(
        &self,
        url: &str,
    ) -> Result<T, DistributorError>
```

## Type Parameters

- `T: for<'de> serde::Deserialize<'de`

## Source
Lines 102–131 in `crates/oxide-library/src/distributors/lcsc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lcsc](/crates/oxide-library/src/distributors/lcsc.md) |
| calls | [Network](/crates/oxide-widgets/src/passive_calculator/network/Network.md) |

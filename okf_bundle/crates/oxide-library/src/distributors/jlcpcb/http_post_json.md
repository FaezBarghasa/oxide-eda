---
okf_version: "0.2"
type: Function
title: http_post_json
resource: crates/oxide-library/src/distributors/jlcpcb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/jlcpcb/http_post_json
language: rust
---

# http_post_json

## Signature

```rust
impl JlcpcbAdapter { fn http_post_json(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T, DistributorError> }
```

## Type Parameters

- `B: serde::Serialize`
- `T: for<'de> Deserialize<'de`

## Source
Lines 72–103 in `crates/oxide-library/src/distributors/jlcpcb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [jlcpcb](/crates/oxide-library/src/distributors/jlcpcb.md) |
| calls | [Network](/crates/oxide-widgets/src/passive_calculator/network/Network.md) |

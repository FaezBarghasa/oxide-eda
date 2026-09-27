---
okf_version: "0.2"
type: Class
title: LcscAdapter
description: LCSC adapter.
resource: crates/oxide-library/src/distributors/lcsc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/lcsc/LcscAdapter
language: rust
---

# LcscAdapter

LCSC adapter.

## Signature

```rust
pub struct LcscAdapter
```

## Visibility

- `pub`

## Docstring

LCSC adapter.

Construct with [`LcscAdapter::new`] for production (default base URL,
blocking reqwest client) or [`LcscAdapter::with_base_url`] for tests
(point at a wiremock server).

## Methods

- `base_url`
- `cache`
- `throttle`
- `http`

## Source
Lines 42–47 in `crates/oxide-library/src/distributors/lcsc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lcsc](/crates/oxide-library/src/distributors/lcsc.md) |

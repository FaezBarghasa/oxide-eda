---
okf_version: "0.2"
type: Function
title: with_base_url
description: "Test constructor: override the base URL (e.g. wiremock)."
resource: crates/oxide-library/src/distributors/lcsc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/lcsc/with_base_url_1
language: rust
---

# with_base_url

Test constructor: override the base URL (e.g. wiremock).

## Signature

```rust
pub fn with_base_url(base_url: impl Into<String>, cache: Option<DistributorCache>) -> Self
```

## Visibility

- `pub`

## Docstring

Test constructor: override the base URL (e.g. wiremock).

## Source
Lines 64–74 in `crates/oxide-library/src/distributors/lcsc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lcsc](/crates/oxide-library/src/distributors/lcsc.md) |

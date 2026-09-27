---
okf_version: "0.2"
type: Function
title: with_api_key
description: "Test constructor: inline API key, override base URL."
resource: crates/oxide-library/src/distributors/mouser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/mouser/with_api_key
language: rust
---

# with_api_key

Test constructor: inline API key, override base URL.

## Signature

```rust
impl MouserAdapter { pub fn with_api_key(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        cache: Option<DistributorCache>,
    ) -> Self }
```

## Visibility

- `pub`

## Docstring

Test constructor: inline API key, override base URL.

## Source
Lines 73–88 in `crates/oxide-library/src/distributors/mouser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mouser](/crates/oxide-library/src/distributors/mouser.md) |

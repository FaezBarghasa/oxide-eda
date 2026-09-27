---
okf_version: "0.2"
type: Function
title: hash_returns_backend_error_on_non_finite_float
description: "`serde_json` cannot encode `NaN` / `±Infinity`. The hash function"
resource: crates/oxide-library/src/hash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/hash/hash_returns_backend_error_on_non_finite_float
language: rust
---

# hash_returns_backend_error_on_non_finite_float

`serde_json` cannot encode `NaN` / `±Infinity`. The hash function

## Signature

```rust
fn hash_returns_backend_error_on_non_finite_float()
```

## Decorators

- `test`

## Docstring

`serde_json` cannot encode `NaN` / `±Infinity`. The hash function
surfaces that as `LibraryError::Backend` instead of panicking.
[test]

## Source
Lines 239–252 in `crates/oxide-library/src/hash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hash](/crates/oxide-library/src/hash.md) |
| calls | [fixture_row](/crates/oxide-library/src/hash/fixture_row.md) |
| calls | [hash_row_content](/crates/oxide-library/src/hash/hash_row_content.md) |

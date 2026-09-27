---
okf_version: "0.2"
type: Function
title: check_param_map_finite
description: "Reject any `ParamValue::Number` / `ParamValue::Measurement` whose float"
resource: crates/oxide-library/src/hash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/hash/check_param_map_finite
language: rust
---

# check_param_map_finite

Reject any `ParamValue::Number` / `ParamValue::Measurement` whose float

## Signature

```rust
fn check_param_map_finite(params: &ParamMap) -> Result<(), LibraryError>
```

## Docstring

Reject any `ParamValue::Number` / `ParamValue::Measurement` whose float
payload is not finite (`NaN`, `Infinity`, `-Infinity`).

## Source
Lines 113–130 in `crates/oxide-library/src/hash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hash](/crates/oxide-library/src/hash.md) |
| called_by | [hash_row_content](/crates/oxide-library/src/hash/hash_row_content.md) |

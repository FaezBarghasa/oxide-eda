---
okf_version: "0.2"
type: Function
title: last_call_age
description: "Helper exposed for tests: how many ms since the last recorded request,"
resource: crates/oxide-library/src/distributors/lcsc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/lcsc/last_call_age
language: rust
---

# last_call_age

Helper exposed for tests: how many ms since the last recorded request,

## Signature

```rust
impl LcscAdapter { pub fn last_call_age(&self) -> Option<Duration> }
```

## Visibility

- `pub`

## Docstring

Helper exposed for tests: how many ms since the last recorded request,
or None if no request has been made.
[doc(hidden)]

## Source
Lines 95–100 in `crates/oxide-library/src/distributors/lcsc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lcsc](/crates/oxide-library/src/distributors/lcsc.md) |

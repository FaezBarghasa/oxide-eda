---
okf_version: "0.2"
type: Function
title: from_standard_str
description: "Parse a Standard `(paper \"...\")` string into a `PageSize`."
resource: crates/oxide-output/src/pdf/page.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/page/from_standard_str
language: rust
---

# from_standard_str

Parse a Standard `(paper "...")` string into a `PageSize`.

## Signature

```rust
impl PageSize { pub fn from_standard_str(s: &str) -> Self }
```

## Visibility

- `pub`

## Docstring

Parse a Standard `(paper "...")` string into a `PageSize`.

Standard uses strings like `"A4"`, `"A3"`, `"A"`, `"B"`, `"USLetter"`,
`"USLegal"`. Unknown strings fall back to `IsoA4`.

## Source
Lines 16–43 in `crates/oxide-output/src/pdf/page.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [page](/crates/oxide-output/src/pdf/page.md) |

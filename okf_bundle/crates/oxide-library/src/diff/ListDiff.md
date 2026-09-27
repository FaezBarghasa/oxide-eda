---
okf_version: "0.2"
type: Class
title: ListDiff
description: "Identity-keyed diff over a list — `String` keys (e.g. `manufacturer:mpn`)."
resource: crates/oxide-library/src/diff.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/diff/ListDiff
language: rust
---

# ListDiff

Identity-keyed diff over a list — `String` keys (e.g. `manufacturer:mpn`).

## Signature

```rust
pub struct ListDiff
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq)`

## Visibility

- `pub`

## Docstring

Identity-keyed diff over a list — `String` keys (e.g. `manufacturer:mpn`).
[derive(Clone, Debug, Default, PartialEq)]

## Methods

- `added`
- `removed`

## Source
Lines 65–68 in `crates/oxide-library/src/diff.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff](/crates/oxide-library/src/diff.md) |

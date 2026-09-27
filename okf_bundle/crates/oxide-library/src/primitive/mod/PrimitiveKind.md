---
okf_version: "0.2"
type: Class
title: PrimitiveKind
description: "Discriminator surfaced on `PrimitiveSummary` so a single `list_*` API can"
resource: crates/oxide-library/src/primitive/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/mod/PrimitiveKind
language: rust
---

# PrimitiveKind

Discriminator surfaced on `PrimitiveSummary` so a single `list_*` API can

## Signature

```rust
pub enum PrimitiveKind
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)`
- `non_exhaustive`

## Visibility

- `pub`

## Docstring

Discriminator surfaced on `PrimitiveSummary` so a single `list_*` API can
describe heterogeneous primitive collections.
[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
[non_exhaustive]

## Source
Lines 39–43 in `crates/oxide-library/src/primitive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitive](/crates/oxide-library/src/primitive/mod.md) |

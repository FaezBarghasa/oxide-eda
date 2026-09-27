---
okf_version: "0.2"
type: Class
title: PrimitiveSummary
description: "Header row for a primitive listing — name + uuid + kind tag, plus a hint"
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/PrimitiveSummary
language: rust
---

# PrimitiveSummary

Header row for a primitive listing — name + uuid + kind tag, plus a hint

## Signature

```rust
pub struct PrimitiveSummary
```

## Decorators

- `derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Header row for a primitive listing — name + uuid + kind tag, plus a hint
of how many rows depend on it (for the library editor's "in use" badge).
The `used_by_count` is a snapshot the adapter computes from its own
state; it's not authoritative across an open `LibrarySet` (resolver
aggregation is the caller's job).
[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `uuid`
- `name`
- `kind`
- `used_by_count`

## Source
Lines 79–85 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |

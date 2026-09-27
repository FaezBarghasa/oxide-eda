---
okf_version: "0.2"
type: Class
title: PrimitiveRef
description: "`(library_id, primitive_uuid)` — the canonical primitive address."
resource: crates/oxide-library/src/primitive/ref_.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/ref_/PrimitiveRef
language: rust
---

# PrimitiveRef

`(library_id, primitive_uuid)` — the canonical primitive address.

## Signature

```rust
pub struct PrimitiveRef
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

`(library_id, primitive_uuid)` — the canonical primitive address.

`Hash + Eq + Ord` so the type works as a key in `HashMap` / `BTreeMap`,
which `WhereUsedIndex` and the resolver both need.
[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]

## Methods

- `library_id`
- `uuid`

## Source
Lines 17–22 in `crates/oxide-library/src/primitive/ref_.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ref_](/crates/oxide-library/src/primitive/ref_.md) |

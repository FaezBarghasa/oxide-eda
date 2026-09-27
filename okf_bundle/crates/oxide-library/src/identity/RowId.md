---
okf_version: "0.2"
type: Class
title: RowId
description: Stable row identifier — UUIDv7 for time-orderability. Newtype wraps
resource: crates/oxide-library/src/identity.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/identity/RowId
language: rust
---

# RowId

Stable row identifier — UUIDv7 for time-orderability. Newtype wraps

## Signature

```rust
pub struct RowId
```

## Decorators

- `derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)`
- `serde(transparent)`

## Visibility

- `pub`

## Docstring

Stable row identifier — UUIDv7 for time-orderability. Newtype wraps
`Uuid` so the type system can distinguish a `RowId` from a generic UUID
used elsewhere (library_id, primitive uuid, etc).

Per `v0.9-refactor-2-plan.md` §6 step 1.10, this replaces the previous
`ComponentId = Uuid` type alias from the v0.9-original layout.
[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
[serde(transparent)]

## Source
Lines 12–12 in `crates/oxide-library/src/identity.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [identity](/crates/oxide-library/src/identity.md) |

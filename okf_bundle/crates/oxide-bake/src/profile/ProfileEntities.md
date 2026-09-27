---
okf_version: "0.2"
type: Class
title: ProfileEntities
description: "Entity-level result of a profile trace — the loop's *topology*."
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile/ProfileEntities
language: rust
---

# ProfileEntities

Entity-level result of a profile trace — the loop's *topology*.

## Signature

```rust
pub struct ProfileEntities
```

## Decorators

- `derive(Clone, Debug, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Entity-level result of a profile trace — the loop's *topology*.

Adjacency and loop closure are pure [`EntityKind`] structure, so
this resolves without a solve. Only turning the loop into positions
needs solved state (see [`trace_closed_profile`]).
[derive(Clone, Debug, PartialEq, Eq)]

## Methods

- `edges`
- `points`

## Source
Lines 86–96 in `crates/oxide-bake/src/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-bake/src/profile.md) |

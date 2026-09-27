---
okf_version: "0.2"
type: Class
title: LinkedRadius
description: v0.24 Track A — Linked-radius semantics for parametric pad corners.
resource: crates/oxide-sketch/src/attr.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-sketch/src/attr/LinkedRadius
language: rust
---

# LinkedRadius

v0.24 Track A — Linked-radius semantics for parametric pad corners.

## Signature

```rust
pub enum LinkedRadius
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "PascalCase")`

## Visibility

- `pub`

## Docstring

v0.24 Track A — Linked-radius semantics for parametric pad corners.

When a RoundRect pad is minted into a sketch, all four corner Arcs
share a single `corner_r_<pad_id>` parameter so changing it updates
every corner in lockstep — Fusion-parity behaviour. The user can
later right-click an individual corner and "Unlink" to override one
or more corners independently; that flips the variant from
`Shared` to `PerCorner`.

Phase 2 (Agent A) attaches this enum to `EntityKind::Arc` (or a
sibling attribute) when A2 (Properties row) and A3 (Unlink) ship.
This A1 phase introduces the type only — the geometry generator
reads radius implicitly through the shared parameter and does not
store a `LinkedRadius` value yet.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "PascalCase")]

## Methods

- `param`
- `ne`
- `se`
- `sw`
- `nw`

## Source
Lines 626–641 in `crates/oxide-sketch/src/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-sketch/src/attr.md) |

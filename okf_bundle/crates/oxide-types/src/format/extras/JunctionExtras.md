---
okf_version: "0.2"
type: Class
title: JunctionExtras
description: "Per-junction auxiliary fields that don't fit into [`SchJunctionRow`] —"
resource: crates/oxide-types/src/format/extras.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/extras/JunctionExtras
language: rust
---

# JunctionExtras

Per-junction auxiliary fields that don't fit into [`SchJunctionRow`] —

## Signature

```rust
pub(in crate::format) struct JunctionExtras
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub(in crate::format)`

## Docstring

Per-junction auxiliary fields that don't fit into [`SchJunctionRow`] —
today just the minted-vs-user-placed provenance bit (issue #422). Only
emitted for a junction where `minted` differs from the on-disk default
(`false`, i.e. user-placed), so a `.snxsch` predating this field round-
trips unchanged and every dot it names loads as user-placed.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `minted`

## Source
Lines 38–41 in `crates/oxide-types/src/format/extras.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [extras](/crates/oxide-types/src/format/extras.md) |

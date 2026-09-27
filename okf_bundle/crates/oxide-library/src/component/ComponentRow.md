---
okf_version: "0.2"
type: Class
title: ComponentRow
description: One row inside a component table — Altium DBLib model.
resource: crates/oxide-library/src/component.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/component/ComponentRow
language: rust
---

# ComponentRow

One row inside a component table — Altium DBLib model.

## Signature

```rust
pub struct ComponentRow
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

One row inside a component table — Altium DBLib model.

Per `v0.9-refactor-2-plan.md` §2.1, a row carries the binding metadata
for a manufacturer part: which primitives (symbol/footprint/sim) it
points at, the parametric data, the supply chain, and the lifecycle
state. The schema is identical across LocalGit (TSV columns) and
Database (JSONB payload) backends — one wire format, two storage
flavours.

Past versions of a row are read from `git log` (LocalGit) or the
audit trail (Database); there is no per-row revision chain anymore.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `row_id`
- `internal_pn`
- `class`
- `datasheet`
- `state`
- `symbol_ref`
- `footprint_ref`
- `sim_ref`
- `pin_map_overrides`
- `primary_mpn`
- `alternates`
- `supply`
- `parameters`
- `plm`
- `version`
- `released`
- `symbol_version`
- `footprint_version`
- `sim_version`
- `created`
- `updated`
- `content_hash`

## Source
Lines 100–177 in `crates/oxide-library/src/component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component](/crates/oxide-library/src/component.md) |

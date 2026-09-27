---
okf_version: "0.2"
type: Function
title: entity_to_graphic
description: "Translate a single edge entity to an `FpGraphic`. Returns"
resource: crates/oxide-bake/src/silk.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/silk/entity_to_graphic
language: rust
---

# entity_to_graphic

Translate a single edge entity to an `FpGraphic`. Returns

## Signature

```rust
fn entity_to_graphic(
    entity: &Entity,
    sketch: &SketchData,
    solve: &FullSolveOutput,
) -> Result<Option<FpGraphic>, String>
```

## Docstring

Translate a single edge entity to an `FpGraphic`. Returns
`Ok(None)` for non-edge entities (Points), `Err(...)` if a
referenced endpoint position can't be resolved.

## Source
Lines 87–149 in `crates/oxide-bake/src/silk.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [silk](/crates/oxide-bake/src/silk.md) |
| calls | [pos](/crates/oxide-bake/src/silk/pos.md) |
| called_by | [bake_silk](/crates/oxide-bake/src/silk/bake_silk.md) |

---
okf_version: "0.2"
type: Function
title: pos
resource: crates/oxide-bake/src/silk.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/silk/pos
language: rust
---

# pos

## Signature

```rust
fn pos(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    id: oxide_sketch::id::SketchEntityId,
) -> Result<[f64; 2], String>
```

## Source
Lines 156–164 in `crates/oxide-bake/src/silk.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [silk](/crates/oxide-bake/src/silk.md) |
| called_by | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
| called_by | [entity_to_graphic](/crates/oxide-bake/src/silk/entity_to_graphic.md) |

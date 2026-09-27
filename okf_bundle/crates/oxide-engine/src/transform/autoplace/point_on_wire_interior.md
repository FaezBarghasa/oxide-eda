---
okf_version: "0.2"
type: Function
title: point_on_wire_interior
description: "---------------------------------------------------------------------------"
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/point_on_wire_interior
language: rust
---

# point_on_wire_interior

---------------------------------------------------------------------------

## Signature

```rust
fn point_on_wire_interior(
    point: oxide_types::schematic::Point,
    wire: &oxide_types::schematic::Wire,
    tolerance: f64,
) -> bool
```

## Docstring

---------------------------------------------------------------------------
Geometry helpers
---------------------------------------------------------------------------

## Source
Lines 311–335 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| called_by | [junctions_under_new_wire](/crates/oxide-engine/src/transform/autoplace/junctions_under_new_wire.md) |
| called_by | [wire_meeting_justifies_junction](/crates/oxide-engine/src/transform/autoplace/wire_meeting_justifies_junction.md) |

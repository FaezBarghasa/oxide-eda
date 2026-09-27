---
okf_version: "0.2"
type: Function
title: needed_junction
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/needed_junction
language: rust
---

# needed_junction

## Signature

```rust
pub(crate) fn needed_junction(
    point: oxide_types::schematic::Point,
    document: &SchematicSheet,
    tolerance: f64,
) -> Option<oxide_types::schematic::Junction>
```

## Visibility

- `pub(crate)`

## Source
Lines 480–498 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| calls | [wire_meeting_justifies_junction](/crates/oxide-engine/src/transform/autoplace/wire_meeting_justifies_junction.md) |
| calls | [junction_at](/crates/oxide-engine/src/transform/autoplace/junction_at.md) |
| called_by | [junctions_for_wire](/crates/oxide-engine/src/transform/autoplace/junctions_for_wire.md) |

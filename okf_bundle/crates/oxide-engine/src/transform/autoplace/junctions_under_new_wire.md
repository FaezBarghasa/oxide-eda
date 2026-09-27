---
okf_version: "0.2"
type: Function
title: junctions_under_new_wire
description: "Junction dots a newly drawn wire needs because an **existing** wire's"
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/junctions_under_new_wire
language: rust
---

# junctions_under_new_wire

Junction dots a newly drawn wire needs because an **existing** wire's

## Signature

```rust
pub(crate) fn junctions_under_new_wire(
    wire: &oxide_types::schematic::Wire,
    document: &SchematicSheet,
    tolerance: f64,
) -> Vec<oxide_types::schematic::Junction>
```

## Visibility

- `pub(crate)`

## Docstring

Junction dots a newly drawn wire needs because an **existing** wire's
endpoint lands on the new wire's interior — the mirror image of
[`needed_junction`], which only ever inspects the *new* wire's own two
endpoints.

Without this, drawing a stub and then a trunk through the stub's endpoint
produced a real junction-less T with no dot. The netlist deliberately treats
that as disconnected (issue #107), so the connection was silently lost
(issue #402). `document` may already contain the new wire; a wire endpoint
can never sit on its own interior, so no self-exclusion is needed.

## Source
Lines 406–428 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| calls | [point_on_wire_interior](/crates/oxide-engine/src/transform/autoplace/point_on_wire_interior.md) |
| calls | [wire_meeting_justifies_junction](/crates/oxide-engine/src/transform/autoplace/wire_meeting_justifies_junction.md) |
| calls | [junction_at](/crates/oxide-engine/src/transform/autoplace/junction_at.md) |
| called_by | [junctions_for_wire](/crates/oxide-engine/src/transform/autoplace/junctions_for_wire.md) |

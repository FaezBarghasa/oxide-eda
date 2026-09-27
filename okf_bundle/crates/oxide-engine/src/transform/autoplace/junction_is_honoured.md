---
okf_version: "0.2"
type: Function
title: junction_is_honoured
description: "True when a dot at `point` would actually merge two wires — i.e. at least"
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/junction_is_honoured
language: rust
---

# junction_is_honoured

True when a dot at `point` would actually merge two wires — i.e. at least

## Signature

```rust
fn junction_is_honoured(point: oxide_types::schematic::Point, document: &SchematicSheet) -> bool
```

## Docstring

True when a dot at `point` would actually merge two wires — i.e. at least
two of the sheet's wires contain `point` **exactly** in the netlist's 1 µm
key space.

The geometry above works in `f64` mm with a 0.01 mm tolerance, but
`SheetConnectivity` honours a junction only where `point_on_segment` holds
with *exact* collinearity in the key space (D5.5). A candidate a few µm off
a wire therefore yields a dot the netlist refuses to act on: a reassuring
visual asserting a connection that does not exist, which is strictly worse
than the undotted T it was meant to fix (issue #402). Off-grid endpoints —
imported or legacy geometry — are exactly where the two metrics diverge, so
every dot this module mints is gated on the netlist's own answer rather than
on the float tolerance alone.

## Source
Lines 350–360 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| calls | [pt_key](/crates/oxide-erc/src/context/pt_key.md) |
| called_by | [wire_meeting_justifies_junction](/crates/oxide-engine/src/transform/autoplace/wire_meeting_justifies_junction.md) |

---
okf_version: "0.2"
type: Function
title: dragging_a_stub_onto_a_trunks_interior_gets_a_junction
description: Dragging a stub onto a trunk is at least as ordinary as drawing through
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/dragging_a_stub_onto_a_trunks_interior_gets_a_junction
language: rust
---

# dragging_a_stub_onto_a_trunks_interior_gets_a_junction

Dragging a stub onto a trunk is at least as ordinary as drawing through

## Signature

```rust
fn dragging_a_stub_onto_a_trunks_interior_gets_a_junction()
```

## Decorators

- `test`

## Docstring

Dragging a stub onto a trunk is at least as ordinary as drawing through
one, and it produced the identical defect: `MoveSelection` mutates wire
coordinates but reconciled no junctions, so the drag landed a real
junction-less T the netlist reads as disconnected (issues #107, #402).
Fixing only `PlaceWireSegment` left this sibling caller broken.
[test]

## Source
Lines 689–721 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
| calls | [test_sheet](/crates/oxide-engine/src/test_support/test_sheet.md) |
| calls | [wire](/crates/oxide-engine/src/lib/wire.md) |

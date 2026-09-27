---
okf_version: "0.2"
type: Function
title: wire_meeting_justifies_junction
description: "True when a genuine wire *meeting* at `point` justifies a junction dot —"
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/wire_meeting_justifies_junction
language: rust
---

# wire_meeting_justifies_junction

True when a genuine wire *meeting* at `point` justifies a junction dot —

## Signature

```rust
pub(crate) fn wire_meeting_justifies_junction(
    point: oxide_types::schematic::Point,
    document: &SchematicSheet,
    tolerance: f64,
) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

True when a genuine wire *meeting* at `point` justifies a junction dot —
a T (one wire terminates here, on the interior of another) or a star
(3+ wires terminate here) — as opposed to two wires' interiors merely
*crossing* at `point` with neither one ending there.

This is the shared gate for both minting a new dot ([`needed_junction`])
and re-validating one that already exists (`reconcile_wire_junctions`,
`transform/mod.rs`): a point no wire terminates at is never a real
connection, no matter how many wire interiors happen to pass through it.
Without the `endpoint_count == 0` guard, a stale dot at a former T can
look "still honoured" once an unrelated wire is later dragged to merely
cross the same point — silently merging two nets the user never
connected (issue #422).

## Source
Lines 462–478 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| calls | [wire_endpoint_count](/crates/oxide-engine/src/transform/autoplace/wire_endpoint_count.md) |
| calls | [point_on_wire_interior](/crates/oxide-engine/src/transform/autoplace/point_on_wire_interior.md) |
| calls | [junction_is_honoured](/crates/oxide-engine/src/transform/autoplace/junction_is_honoured.md) |
| called_by | [junctions_under_new_wire](/crates/oxide-engine/src/transform/autoplace/junctions_under_new_wire.md) |
| called_by | [needed_junction](/crates/oxide-engine/src/transform/autoplace/needed_junction.md) |
| called_by | [remove_unjustified_minted_junctions](/crates/oxide-engine/src/transform/mod/remove_unjustified_minted_junctions.md) |

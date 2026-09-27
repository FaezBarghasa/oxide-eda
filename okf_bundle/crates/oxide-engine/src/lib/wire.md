---
okf_version: "0.2"
type: Function
title: wire
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/wire
language: rust
---

# wire

## Signature

```rust
fn wire(a: Point, b: Point) -> oxide_types::schematic::Wire
```

## Source
Lines 627–634 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
| called_by | [a_trunk_crossing_another_wires_interior_gets_no_junction](/crates/oxide-engine/src/lib/a_trunk_crossing_another_wires_interior_gets_no_junction.md) |
| called_by | [a_trunk_drawn_through_an_existing_wires_endpoint_gets_a_junction](/crates/oxide-engine/src/lib/a_trunk_drawn_through_an_existing_wires_endpoint_gets_a_junction.md) |
| called_by | [a_user_placed_junction_survives_reconcile_even_when_unjustified](/crates/oxide-engine/src/lib/a_user_placed_junction_survives_reconcile_even_when_unjustified.md) |
| called_by | [an_endpoint_off_the_trunk_in_key_space_gets_no_lying_junction](/crates/oxide-engine/src/lib/an_endpoint_off_the_trunk_in_key_space_gets_no_lying_junction.md) |
| called_by | [deleting_a_stub_removes_the_stale_dot_so_a_later_crossing_wire_cannot_merge_nets](/crates/oxide-engine/src/lib/deleting_a_stub_removes_the_stale_dot_so_a_later_crossing_wire_cannot_merge_nets.md) |
| called_by | [dragging_a_minted_stub_off_the_trunk_removes_the_stale_dot](/crates/oxide-engine/src/lib/dragging_a_minted_stub_off_the_trunk_removes_the_stale_dot.md) |
| called_by | [dragging_a_stub_across_a_trunk_gets_no_junction](/crates/oxide-engine/src/lib/dragging_a_stub_across_a_trunk_gets_no_junction.md) |
| called_by | [dragging_a_stub_onto_a_trunks_interior_gets_a_junction](/crates/oxide-engine/src/lib/dragging_a_stub_onto_a_trunks_interior_gets_a_junction.md) |

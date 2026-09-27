---
okf_version: "0.2"
type: Module
title: sim
description: Sim tab — SPICE deck editor + per-pin SPICE node mapping.
resource: crates/oxide-app/src/library/editor/sim/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:12:05Z"
concept_id: crates/oxide-app/src/library/editor/sim/mod
language: rust
---

# sim

Sim tab — SPICE deck editor + per-pin SPICE node mapping.

## Docstring

Sim tab — SPICE deck editor + per-pin SPICE node mapping.

Backed by the typed `SimModel` primitive bound through
`ComponentRow::sim_ref`. The view operates on `state.sim`
(resolved lazily by the dispatcher's `PreviewTab::Simulation` arm
via `LibrarySet::resolve_sim`) and `state.sim_body`, the live
`text_editor::Content` mirror of the SPICE deck.

The pin/node table iterates `state.symbol.pins`; the SPICE-node
buffer for a given pin is read from
`sim.default_node_map[pin_number]` and edits flow back through
[`EditorMsg::SimSetPinNode`] which empty-removes / non-empty-
inserts the mapping.

## Relationships

| Type | Target |
|------|--------|
| related | [SimKindPick](/crates/oxide-app/src/library/editor/sim/mod/SimKindPick.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/sim/mod/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/sim/mod/fmt.md) |
| related | [view](/crates/oxide-app/src/library/editor/sim/mod/view.md) |
| related | [view_pin_node_table](/crates/oxide-app/src/library/editor/sim/mod/view_pin_node_table.md) |
| related | [pin_node_row](/crates/oxide-app/src/library/editor/sim/mod/pin_node_row.md) |
| related | [fixture_editor](/crates/oxide-app/src/library/editor/sim/mod/fixture_editor.md) |
| related | [apply](/crates/oxide-app/src/library/editor/sim/mod/apply.md) |
| related | [sim_set_enabled_round_trip_clears_state](/crates/oxide-app/src/library/editor/sim/mod/sim_set_enabled_round_trip_clears_state.md) |
| related | [sim_set_pin_node_persists_into_default_node_map](/crates/oxide-app/src/library/editor/sim/mod/sim_set_pin_node_persists_into_default_node_map.md) |
| related | [sim_set_pin_node_empty_removes_key](/crates/oxide-app/src/library/editor/sim/mod/sim_set_pin_node_empty_removes_key.md) |
| related | [sim_set_pin_node_whitespace_removes_key](/crates/oxide-app/src/library/editor/sim/mod/sim_set_pin_node_whitespace_removes_key.md) |
| related | [sim_set_kind_and_name_mutate_in_place](/crates/oxide-app/src/library/editor/sim/mod/sim_set_kind_and_name_mutate_in_place.md) |
| related | [sim_set_enabled_true_is_idempotent_when_already_bound](/crates/oxide-app/src/library/editor/sim/mod/sim_set_enabled_true_is_idempotent_when_already_bound.md) |
| related | [iced](/_dependencies/cargo/iced.md) |

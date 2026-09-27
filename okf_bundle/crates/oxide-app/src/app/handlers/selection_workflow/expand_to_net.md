---
okf_version: "0.2"
type: Function
title: expand_to_net
description: "Given a hit on a wire/bus/junction/label, return the full set of net-geom"
resource: crates/oxide-app/src/app/handlers/selection_workflow.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/selection_workflow/expand_to_net
language: rust
---

# expand_to_net

Given a hit on a wire/bus/junction/label, return the full set of net-geom

## Signature

```rust
fn expand_to_net(
    snapshot: &crate::schematic_runtime::SchematicRenderSnapshot,
    seed: &oxide_types::schematic::SelectedItem,
) -> Vec<oxide_types::schematic::SelectedItem>
```

## Docstring

Given a hit on a wire/bus/junction/label, return the full set of net-geom
items (wires, buses, junctions, labels) reachable by shared endpoints.
Symbols and their pins are intentionally excluded — this matches Altium's
"Select » Connection" behaviour which picks net geometry only.

Endpoints are quantised to 0.001 mm (1 nm in Standard-space integer units)
and stored in a HashSet — lookup is O(1), so the overall walk is
O(N · passes) instead of the naive O(P²·N²). Critical for power nets
with hundreds of wires.

## Source
Lines 225–337 in `crates/oxide-app/src/app/handlers/selection_workflow.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection_workflow](/crates/oxide-app/src/app/handlers/selection_workflow.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [handle_selection_request](/crates/oxide-app/src/app/handlers/selection_workflow/handle_selection_request.md) |

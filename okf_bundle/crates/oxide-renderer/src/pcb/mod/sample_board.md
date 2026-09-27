---
okf_version: "0.2"
type: Function
title: sample_board
resource: crates/oxide-renderer/src/pcb/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/pcb/mod/sample_board
language: rust
---

# sample_board

## Signature

```rust
fn sample_board() -> PcbBoard
```

## Source
Lines 274–390 in `crates/oxide-renderer/src/pcb/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb](/crates/oxide-renderer/src/pcb/mod.md) |
| called_by | [pcb_drc_violation_type_expands_overlay_line_patterns](/crates/oxide-renderer/src/pcb/mod/pcb_drc_violation_type_expands_overlay_line_patterns.md) |
| called_by | [pcb_overlay_slice_emits_ratsnest_and_drc_primitives](/crates/oxide-renderer/src/pcb/mod/pcb_overlay_slice_emits_ratsnest_and_drc_primitives.md) |
| called_by | [pcb_renderer_updates_each_family_with_matching_dirty_flag](/crates/oxide-renderer/src/pcb/mod/pcb_renderer_updates_each_family_with_matching_dirty_flag.md) |
| called_by | [pcb_snapshot_collects_trace_via_pad_and_zone_inputs](/crates/oxide-renderer/src/pcb/mod/pcb_snapshot_collects_trace_via_pad_and_zone_inputs.md) |

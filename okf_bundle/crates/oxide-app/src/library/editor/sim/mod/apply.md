---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/sim/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:12:05Z"
concept_id: crates/oxide-app/src/library/editor/sim/mod/apply
language: rust
---

# apply

## Signature

```rust
fn apply(editor: &mut ComponentPreviewState, msg: EditorMsg)
```

## Source
Lines 363–369 in `crates/oxide-app/src/library/editor/sim/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/editor/sim/mod.md) |
| calls | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
| called_by | [sim_set_enabled_round_trip_clears_state](/crates/oxide-app/src/library/editor/sim/mod/sim_set_enabled_round_trip_clears_state.md) |
| called_by | [sim_set_enabled_true_is_idempotent_when_already_bound](/crates/oxide-app/src/library/editor/sim/mod/sim_set_enabled_true_is_idempotent_when_already_bound.md) |
| called_by | [sim_set_kind_and_name_mutate_in_place](/crates/oxide-app/src/library/editor/sim/mod/sim_set_kind_and_name_mutate_in_place.md) |
| called_by | [sim_set_pin_node_empty_removes_key](/crates/oxide-app/src/library/editor/sim/mod/sim_set_pin_node_empty_removes_key.md) |
| called_by | [sim_set_pin_node_persists_into_default_node_map](/crates/oxide-app/src/library/editor/sim/mod/sim_set_pin_node_persists_into_default_node_map.md) |
| called_by | [sim_set_pin_node_whitespace_removes_key](/crates/oxide-app/src/library/editor/sim/mod/sim_set_pin_node_whitespace_removes_key.md) |

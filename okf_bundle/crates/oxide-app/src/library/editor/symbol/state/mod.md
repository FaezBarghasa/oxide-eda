---
okf_version: "0.2"
type: Module
title: state
description: Symbol-tab editor state.
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod
language: rust
---

# state

Symbol-tab editor state.

## Docstring

Symbol-tab editor state.

The editor mutates a typed [`oxide_library::Symbol`] primitive
in-place. Helpers below operate on a `&mut Symbol` so the
dispatcher can call them directly off the active editor state.

Selection / hit-test / pin-add / move / delete logic preserves
the canvas + AI-stub apply behaviour the pre-refactor `SymbolDoc`
had.

## Relationships

| Type | Target |
|------|--------|
| related | [PinKind](/crates/oxide-app/src/library/editor/symbol/state/mod/PinKind.md) |
| related | [from_ai_stub](/crates/oxide-app/src/library/editor/symbol/state/mod/from_ai_stub.md) |
| related | [from_ai_stub](/crates/oxide-app/src/library/editor/symbol/state/mod/from_ai_stub.md) |
| related | [FieldKey](/crates/oxide-app/src/library/editor/symbol/state/mod/FieldKey.md) |
| related | [SymbolSelection](/crates/oxide-app/src/library/editor/symbol/state/mod/SymbolSelection.md) |
| related | [BoxSelectKind](/crates/oxide-app/src/library/editor/symbol/state/mod/BoxSelectKind.md) |
| related | [GraphicRotationPivotMode](/crates/oxide-app/src/library/editor/symbol/state/mod/GraphicRotationPivotMode.md) |
| related | [GraphicHandle](/crates/oxide-app/src/library/editor/symbol/state/mod/GraphicHandle.md) |
| related | [handle_interaction](/crates/oxide-app/src/library/editor/symbol/state/mod/handle_interaction.md) |
| related | [SymActiveBarMenu](/crates/oxide-app/src/library/editor/symbol/state/mod/SymActiveBarMenu.md) |
| related | [SymbolSelectionFilter](/crates/oxide-app/src/library/editor/symbol/state/mod/SymbolSelectionFilter.md) |
| related | [default](/crates/oxide-app/src/library/editor/symbol/state/mod/default.md) |
| related | [default](/crates/oxide-app/src/library/editor/symbol/state/mod/default.md) |
| related | [SymbolFilterKind](/crates/oxide-app/src/library/editor/symbol/state/mod/SymbolFilterKind.md) |
| related | [label](/crates/oxide-app/src/library/editor/symbol/state/mod/label.md) |
| related | [label](/crates/oxide-app/src/library/editor/symbol/state/mod/label.md) |
| related | [get](/crates/oxide-app/src/library/editor/symbol/state/mod/get.md) |
| related | [toggle](/crates/oxide-app/src/library/editor/symbol/state/mod/toggle.md) |
| related | [get](/crates/oxide-app/src/library/editor/symbol/state/mod/get.md) |
| related | [toggle](/crates/oxide-app/src/library/editor/symbol/state/mod/toggle.md) |
| related | [max_part_number](/crates/oxide-app/src/library/editor/symbol/state/mod/max_part_number.md) |
| related | [delete_unit](/crates/oxide-app/src/library/editor/symbol/state/mod/delete_unit.md) |
| related | [graphic_on_part](/crates/oxide-app/src/library/editor/symbol/state/mod/graphic_on_part.md) |
| related | [graphic_is_selected](/crates/oxide-app/src/library/editor/symbol/state/mod/graphic_is_selected.md) |
| related | [polygon_centroid](/crates/oxide-app/src/library/editor/symbol/state/mod/polygon_centroid.md) |
| related | [polygon_vertex_mean](/crates/oxide-app/src/library/editor/symbol/state/mod/polygon_vertex_mean.md) |
| related | [pin_on_part](/crates/oxide-app/src/library/editor/symbol/state/mod/pin_on_part.md) |
| related | [join_source_indices](/crates/oxide-app/src/library/editor/symbol/state/mod/join_source_indices.md) |
| related | [selection_kinds_are_line_or_arc](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_kinds_are_line_or_arc.md) |
| related | [common_graphic_part_number](/crates/oxide-app/src/library/editor/symbol/state/mod/common_graphic_part_number.md) |
| related | [selection_has_enough_join_sources](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_has_enough_join_sources.md) |
| related | [selection_is_join_eligible](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_is_join_eligible.md) |
| related | [selected_is_deletable](/crates/oxide-app/src/library/editor/symbol/state/mod/selected_is_deletable.md) |
| related | [selected_is_alignable](/crates/oxide-app/src/library/editor/symbol/state/mod/selected_is_alignable.md) |
| related | [add_pin](/crates/oxide-app/src/library/editor/symbol/state/mod/add_pin.md) |
| related | [next_pin_number](/crates/oxide-app/src/library/editor/symbol/state/mod/next_pin_number.md) |
| related | [iced](/_dependencies/cargo/iced.md) |

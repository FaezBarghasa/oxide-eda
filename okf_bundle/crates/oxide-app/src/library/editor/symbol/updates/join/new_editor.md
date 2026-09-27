---
okf_version: "0.2"
type: Function
title: new_editor
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/new_editor
language: rust
---

# new_editor

## Signature

```rust
fn new_editor() -> SymEditor
```

## Source
Lines 198–203 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| called_by | [all_selection_resolves_to_every_visible_graphic_and_joins](/crates/oxide-app/src/library/editor/symbol/updates/join/all_selection_resolves_to_every_visible_graphic_and_joins.md) |
| called_by | [arc_and_lines_join](/crates/oxide-app/src/library/editor/symbol/updates/join/arc_and_lines_join.md) |
| called_by | [branching_selection_errors_with_no_mutation](/crates/oxide-app/src/library/editor/symbol/updates/join/branching_selection_errors_with_no_mutation.md) |
| called_by | [empty_selection_is_a_no_op](/crates/oxide-app/src/library/editor/symbol/updates/join/empty_selection_is_a_no_op.md) |
| called_by | [mixed_shared_and_unit_specific_selection_is_ineligible](/crates/oxide-app/src/library/editor/symbol/updates/join/mixed_shared_and_unit_specific_selection_is_ineligible.md) |
| called_by | [open_three_side_chain_auto_closes](/crates/oxide-app/src/library/editor/symbol/updates/join/open_three_side_chain_auto_closes.md) |
| called_by | [selection_with_rectangle_is_a_no_op](/crates/oxide-app/src/library/editor/symbol/updates/join/selection_with_rectangle_is_a_no_op.md) |
| called_by | [single_arc_selection_is_eligible_and_self_closes](/crates/oxide-app/src/library/editor/symbol/updates/join/single_arc_selection_is_eligible_and_self_closes.md) |
| called_by | [single_line_selection_is_ineligible_and_silent](/crates/oxide-app/src/library/editor/symbol/updates/join/single_line_selection_is_ineligible_and_silent.md) |
| called_by | [square_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/square_editor.md) |
| called_by | [stroke_width_is_max_of_sources](/crates/oxide-app/src/library/editor/symbol/updates/join/stroke_width_is_max_of_sources.md) |

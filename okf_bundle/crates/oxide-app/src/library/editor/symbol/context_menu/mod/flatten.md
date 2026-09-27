---
okf_version: "0.2"
type: Function
title: flatten
description: "Walk the declarative row tree into the flat `Vec<DropdownEntry>`"
resource: crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten
language: rust
---

# flatten

Walk the declarative row tree into the flat `Vec<DropdownEntry>`

## Signature

```rust
fn flatten(
    rows: Vec<SymbolMenuRow>,
    open_submenu: Option<SymbolContextSubmenu>,
    path: &Path,
    indented: bool,
) -> Vec<DropdownEntry<LibraryMessage>>
```

## Docstring

Walk the declarative row tree into the flat `Vec<DropdownEntry>`
the generic renderer draws — the one place this module converts
pure row data into the shared widget vocabulary. A submenu's
children render directly below their header (indented via a
leading marker — the shared widget has no dedicated indent slot)
only while `open_submenu` names that header, matching the
footprint context menu's accordion-in-place submenu behaviour
(not a hover flyout).

## Source
Lines 62–82 in `crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/context_menu/mod.md) |
| calls | [row_submenu_matches](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/row_submenu_matches.md) |
| calls | [submenu_header_entry](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/submenu_header_entry.md) |
| calls | [leaf_entry](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/leaf_entry.md) |
| called_by | [placement_field_buf](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/placement_field_buf.md) |
| called_by | [owned_sketch_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/owned_sketch_entities.md) |
| called_by | [items](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/items.md) |
| called_by | [carry_links_by_unique_number](/crates/oxide-app/src/library/editor/footprint/state/pad/carry_links_by_unique_number.md) |
| called_by | [align_confirm](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_confirm.md) |
| called_by | [rounded_rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rounded_rectangle.md) |
| called_by | [resolve_effective_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_effective_click.md) |
| called_by | [view_context_menu](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/view_context_menu.md) |
| called_by | [has_stray_tmp](/crates/oxide-app/src/test_support/has_stray_tmp.md) |
| called_by | [collect_rust_files](/crates/oxide-renderer/tests/no_literal_colors/collect_rust_files.md) |
| called_by | [solve](/crates/oxide-widgets/src/passive_calculator/solver/solve.md) |
| called_by | [exhaustive_best_error](/crates/oxide-widgets/tests/passive_calculator/solver_tests/exhaustive_best_error.md) |

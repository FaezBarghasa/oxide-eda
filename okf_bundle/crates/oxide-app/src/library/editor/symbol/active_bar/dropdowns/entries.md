---
okf_version: "0.2"
type: Function
title: entries
description: "Build the entries for the dropdown matching `menu`. `tid` resolves"
resource: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/entries
language: rust
---

# entries

Build the entries for the dropdown matching `menu`. `tid` resolves

## Signature

```rust
pub fn entries(
    menu: SymActiveBarMenu,
    selection_filter: SymbolSelectionFilter,
    active_tool: SymbolTool,
    path: PathBuf,
    tid: ThemeId,
) -> Vec<DropdownEntry<LibraryMessage>>
```

## Visibility

- `pub`

## Docstring

Build the entries for the dropdown matching `menu`. `tid` resolves
the per-theme accent tint on each SVG icon (icons reuse the
schematic active bar's icon set for visual consistency across
editors).

## Source
Lines 53–70 in `crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdowns](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.md) |
| calls | [filter_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/filter_entries.md) |
| calls | [snap_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/snap_entries.md) |
| calls | [place_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/place_entries.md) |
| calls | [select_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/select_entries.md) |
| calls | [align_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/align_entries.md) |
| calls | [pin_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/pin_entries.md) |
| calls | [text_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/text_entries.md) |
| calls | [shapes_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/shapes_entries.md) |

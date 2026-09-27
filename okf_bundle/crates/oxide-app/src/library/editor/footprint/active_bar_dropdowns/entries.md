---
okf_version: "0.2"
type: Function
title: entries
description: "Build the entries for the dropdown matching `menu`. `tid` resolves"
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/entries
language: rust
---

# entries

Build the entries for the dropdown matching `menu`. `tid` resolves

## Signature

```rust
pub fn entries(
    menu: FpActiveBarMenu,
    state: &FootprintEditorState,
    path: PathBuf,
    tid: ThemeId,
    footprint_presets: &[crate::active_bar::FootprintFilterPreset],
) -> Vec<DropdownEntry<LibraryMessage>>
```

## Visibility

- `pub`

## Docstring

Build the entries for the dropdown matching `menu`. `tid` resolves
the per-theme accent tint on each SVG icon (icons are reused from
the schematic active bar's icon set for visual consistency).
`footprint_presets` are the named multi-preset shortcuts shown on
row 1 of the Filter dropdown — footprint-native presets keyed on
`SelectionFilterKind` (Task 6), not the schematic
`CustomFilterPreset`.

## Source
Lines 80–99 in `crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar_dropdowns](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.md) |
| calls | [filter_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/filter_entries.md) |
| calls | [snap_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/snap_entries.md) |
| calls | [place_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/place_entries.md) |
| calls | [select_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/select_entries.md) |
| calls | [align_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/align_entries.md) |
| calls | [body3d_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/body3d_entries.md) |
| calls | [text_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/text_entries.md) |
| calls | [shapes_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/shapes_entries.md) |
| calls | [sketch_create_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/sketch_create_entries.md) |
| calls | [sketch_modify_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/sketch_modify_entries.md) |

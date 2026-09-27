---
okf_version: "0.2"
type: Function
title: filter_entries
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/filter_entries
language: rust
---

# filter_entries

## Signature

```rust
fn filter_entries(
    state: &FootprintEditorState,
    path: PathBuf,
    footprint_presets: &[crate::active_bar::FootprintFilterPreset],
) -> Vec<DropdownEntry<LibraryMessage>>
```

## Source
Lines 210–324 in `crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar_dropdowns](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.md) |
| calls | [chip_btn](/crates/oxide-widgets/src/active_bar/dropdown/chip_btn.md) |
| called_by | [entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/entries.md) |

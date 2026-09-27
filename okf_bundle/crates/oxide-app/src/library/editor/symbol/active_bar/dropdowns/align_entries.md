---
okf_version: "0.2"
type: Function
title: align_entries
resource: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/align_entries
language: rust
---

# align_entries

## Signature

```rust
fn align_entries(path: PathBuf, tid: ThemeId) -> Vec<DropdownEntry<LibraryMessage>>
```

## Source
Lines 255–317 in `crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdowns](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.md) |
| called_by | [align_to_grid_row_snaps_selection](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/align_to_grid_row_snaps_selection.md) |
| called_by | [entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/entries.md) |

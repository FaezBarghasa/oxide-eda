---
okf_version: "0.2"
type: Function
title: snap_entries
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/snap_entries
language: rust
---

# snap_entries

## Signature

```rust
fn snap_entries(state: &FootprintEditorState, path: PathBuf) -> Vec<DropdownEntry<LibraryMessage>>
```

## Source
Lines 326–427 in `crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar_dropdowns](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.md) |
| calls | [fp](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/fp.md) |
| called_by | [entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/entries.md) |

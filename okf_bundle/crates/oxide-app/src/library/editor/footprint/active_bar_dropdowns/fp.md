---
okf_version: "0.2"
type: Function
title: fp
description: "Convenience: route a `FootprintEditorMsg` to the editor at `path`."
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/fp
language: rust
---

# fp

Convenience: route a `FootprintEditorMsg` to the editor at `path`.

## Signature

```rust
fn fp(path: PathBuf, msg: FootprintEditorMsg) -> LibraryMessage
```

## Docstring

Convenience: route a `FootprintEditorMsg` to the editor at `path`.

## Source
Lines 32–37 in `crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar_dropdowns](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.md) |
| called_by | [align_item](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/align_item.md) |
| called_by | [align_item_with_icon](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/align_item_with_icon.md) |
| called_by | [place_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/place_entries.md) |
| called_by | [shapes_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/shapes_entries.md) |
| called_by | [sketch_create_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/sketch_create_entries.md) |
| called_by | [sketch_modify_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/sketch_modify_entries.md) |
| called_by | [snap_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/snap_entries.md) |
| called_by | [stub](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/stub.md) |

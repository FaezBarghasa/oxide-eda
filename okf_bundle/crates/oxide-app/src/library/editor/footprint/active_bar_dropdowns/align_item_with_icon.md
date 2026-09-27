---
okf_version: "0.2"
type: Function
title: align_item_with_icon
description: v0.14 — real Align/Distribute item with an icon.
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/align_item_with_icon
language: rust
---

# align_item_with_icon

v0.14 — real Align/Distribute item with an icon.

## Signature

```rust
fn align_item_with_icon(
    label: &'static str,
    path: PathBuf,
    op: crate::library::editor::footprint::state::AlignOp,
    icon: iced::widget::svg::Handle,
) -> DropdownItem<LibraryMessage>
```

## Docstring

v0.14 — real Align/Distribute item with an icon.

## Source
Lines 64–71 in `crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar_dropdowns](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.md) |
| calls | [fp](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/fp.md) |

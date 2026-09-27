---
okf_version: "0.2"
type: Function
title: align_item
description: "v0.14 — real Align/Distribute/Spacing item, no icon. Emits"
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/align_item
language: rust
---

# align_item

v0.14 — real Align/Distribute/Spacing item, no icon. Emits

## Signature

```rust
fn align_item(
    label: &'static str,
    path: PathBuf,
    op: crate::library::editor::footprint::state::AlignOp,
) -> DropdownItem<LibraryMessage>
```

## Docstring

v0.14 — real Align/Distribute/Spacing item, no icon. Emits
[`FootprintEditorMsg::AlignPads`] so the dispatcher
transforms the current pad selection.

## Source
Lines 55–61 in `crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar_dropdowns](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.md) |
| calls | [fp](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/fp.md) |

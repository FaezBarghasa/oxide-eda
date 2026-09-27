---
okf_version: "0.2"
type: Function
title: sketch_create_entries
description: Sketch ▸ Create — the six geometry tools that used to sit as six
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/sketch_create_entries
language: rust
---

# sketch_create_entries

Sketch ▸ Create — the six geometry tools that used to sit as six

## Signature

```rust
fn sketch_create_entries(
    state: &FootprintEditorState,
    path: PathBuf,
    tid: ThemeId,
) -> Vec<DropdownEntry<LibraryMessage>>
```

## Docstring

Sketch ▸ Create — the six geometry tools that used to sit as six
separate always-visible buttons on the sketch bar. Each row arms
the tool via [`FootprintEditorMsg::ActiveBarSetSketchTool`], which
also dismisses the menu; the armed one carries a checkmark so the
user can see what's in hand without closing the menu first.

## Source
Lines 106–146 in `crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar_dropdowns](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.md) |
| calls | [fp](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/fp.md) |
| called_by | [entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/entries.md) |

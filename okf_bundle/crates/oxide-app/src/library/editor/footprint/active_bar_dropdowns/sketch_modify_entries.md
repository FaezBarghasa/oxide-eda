---
okf_version: "0.2"
type: Function
title: sketch_modify_entries
description: Sketch ▸ Modify — the six edit tools plus the one-shot Make Pad
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/sketch_modify_entries
language: rust
---

# sketch_modify_entries

Sketch ▸ Modify — the six edit tools plus the one-shot Make Pad

## Signature

```rust
fn sketch_modify_entries(
    state: &FootprintEditorState,
    path: PathBuf,
    tid: ThemeId,
) -> Vec<DropdownEntry<LibraryMessage>>
```

## Docstring

Sketch ▸ Modify — the six edit tools plus the one-shot Make Pad
action. Mirror / Offset / the two Pattern tools consume a
selection, so they grey out with an explanatory label when nothing
is selected rather than arming a tool that would only warn.

## Source
Lines 152–208 in `crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar_dropdowns](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.md) |
| calls | [fp](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/fp.md) |
| called_by | [entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/entries.md) |

---
okf_version: "0.2"
type: Function
title: set_selection_mode_2d
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection/set_selection_mode_2d
language: rust
---

# set_selection_mode_2d

## Signature

```rust
fn set_selection_mode_2d(
    editor: &mut crate::app::FootprintEditorState,
    mode: crate::library::editor::footprint::state::FpSelectionMode,
)
```

## Source
Lines 245–254 in `crates/oxide-app/src/library/editor/footprint/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/updates/selection.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/selection/apply.md) |

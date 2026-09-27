---
okf_version: "0.2"
type: Function
title: toggle_selection_filter
description: v0.18.14 — Selection Filter pill toggle from the unified
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection/toggle_selection_filter
language: rust
---

# toggle_selection_filter

v0.18.14 — Selection Filter pill toggle from the unified

## Signature

```rust
fn toggle_selection_filter(
    editor: &mut crate::app::FootprintEditorState,
    kind: crate::library::editor::footprint::state::SelectionFilterKind,
)
```

## Docstring

v0.18.14 — Selection Filter pill toggle from the unified
active bar. The panel-side equivalent
(`PanelMsg::FpEditorToggleSelectionFilter`) routes through
a dedicated handler in `handlers/dock/sch_library`; this
arm covers the active-bar dispatch path.

## Source
Lines 70–76 in `crates/oxide-app/src/library/editor/footprint/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/updates/selection.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/selection/apply.md) |

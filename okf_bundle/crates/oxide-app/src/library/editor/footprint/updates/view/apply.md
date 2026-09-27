---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/view/apply
language: rust
---

# apply

## Signature

```rust
pub(super) fn apply(editor: &mut crate::app::FootprintEditorState, msg: FootprintEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 13–33 in `crates/oxide-app/src/library/editor/footprint/updates/view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/editor/footprint/updates/view.md) |
| calls | [toggle_layer](/crates/oxide-app/src/library/editor/footprint/updates/view/toggle_layer.md) |
| calls | [toggle_auto_fit](/crates/oxide-app/src/library/editor/footprint/updates/view/toggle_auto_fit.md) |
| calls | [set_mode](/crates/oxide-app/src/library/editor/footprint/updates/view/set_mode.md) |
| calls | [toggle_placement_pause](/crates/oxide-app/src/library/editor/footprint/updates/view/toggle_placement_pause.md) |
| calls | [fit_consumed](/crates/oxide-app/src/library/editor/footprint/updates/view/fit_consumed.md) |
| calls | [set_pads_tool](/crates/oxide-app/src/library/editor/footprint/updates/view/set_pads_tool.md) |
| calls | [tool_escape](/crates/oxide-app/src/library/editor/footprint/updates/view/tool_escape.md) |
| calls | [align_pads_action](/crates/oxide-app/src/library/editor/footprint/updates/view/align_pads_action.md) |
| calls | [set_name](/crates/oxide-app/src/library/editor/footprint/updates/view/set_name.md) |
| calls | [recompute_courtyard_outline](/crates/oxide-app/src/library/editor/footprint/updates/view/recompute_courtyard_outline.md) |

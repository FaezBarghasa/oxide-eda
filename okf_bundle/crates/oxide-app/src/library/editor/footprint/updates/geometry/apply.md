---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/apply
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
Lines 11–35 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| calls | [add_new_sibling](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_new_sibling.md) |
| calls | [add_pad](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_pad.md) |
| calls | [add_via](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_via.md) |
| calls | [track_click](/crates/oxide-app/src/library/editor/footprint/updates/geometry/track_click.md) |
| calls | [track_cancel](/crates/oxide-app/src/library/editor/footprint/updates/geometry/track_cancel.md) |
| calls | [arc_click](/crates/oxide-app/src/library/editor/footprint/updates/geometry/arc_click.md) |
| calls | [arc_cancel](/crates/oxide-app/src/library/editor/footprint/updates/geometry/arc_cancel.md) |
| calls | [polygon_click](/crates/oxide-app/src/library/editor/footprint/updates/geometry/polygon_click.md) |
| calls | [polygon_commit](/crates/oxide-app/src/library/editor/footprint/updates/geometry/polygon_commit.md) |
| calls | [polygon_cancel](/crates/oxide-app/src/library/editor/footprint/updates/geometry/polygon_cancel.md) |
| calls | [add_text](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_text.md) |
| calls | [add_text_frame](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_text_frame.md) |
| calls | [add_hole](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_hole.md) |
| calls | [mint_body3d](/crates/oxide-app/src/library/editor/footprint/updates/geometry/mint_body3d.md) |
| calls | [mint_extruded_body3d](/crates/oxide-app/src/library/editor/footprint/updates/geometry/mint_extruded_body3d.md) |

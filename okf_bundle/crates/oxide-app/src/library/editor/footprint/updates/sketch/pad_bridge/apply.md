---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/apply
language: rust
---

# apply

## Signature

```rust
pub(in crate::library::editor::footprint::updates) fn apply(
    editor: &mut crate::app::FootprintEditorState,
    msg: FootprintEditorMsg,
)
```

## Visibility

- `pub(in crate::library::editor::footprint::updates)`

## Source
Lines 9–23 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_bridge](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.md) |
| calls | [set_role](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/set_role.md) |
| calls | [make_pad_from_profile](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/make_pad_from_profile.md) |
| calls | [unlink_corner_radius](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/unlink_corner_radius.md) |

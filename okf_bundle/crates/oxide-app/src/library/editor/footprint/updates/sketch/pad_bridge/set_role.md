---
okf_version: "0.2"
type: Function
title: set_role
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/set_role
language: rust
---

# set_role

## Signature

```rust
fn set_role(
    editor: &mut crate::app::FootprintEditorState,
    id: oxide_sketch::id::SketchEntityId,
    role: crate::library::messages::RoleTag,
)
```

## Source
Lines 25–123 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_bridge](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.md) |
| calls | [apply_sketch_role_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_role_with_warnings.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/apply.md) |

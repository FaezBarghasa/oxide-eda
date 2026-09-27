---
okf_version: "0.2"
type: Function
title: current_role_of
description: "Read the current [`RoleTag`] of an entity by inspecting which"
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch/current_role_of
language: rust
---

# current_role_of

Read the current [`RoleTag`] of an entity by inspecting which

## Signature

```rust
pub fn current_role_of(entity: &oxide_sketch::entity::Entity) -> RoleTag
```

## Visibility

- `pub`

## Docstring

Read the current [`RoleTag`] of an entity by inspecting which
`*Attr` slot is populated. Returns `RoleTag::Unassigned` when no
role attr is set (the default for fresh entities). Used by the
inspector to highlight the active dropdown value.

## Source
Lines 258–309 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch.md) |
| called_by | [build_footprint_editor_panel_ctx](/crates/oxide-app/src/app/runtime/footprint_ctx/build_footprint_editor_panel_ctx.md) |
| called_by | [view_role](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_role.md) |

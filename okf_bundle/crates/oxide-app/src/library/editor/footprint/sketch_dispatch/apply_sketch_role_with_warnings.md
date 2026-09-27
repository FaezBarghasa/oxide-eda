---
okf_version: "0.2"
type: Function
title: apply_sketch_role_with_warnings
description: "`_with_warnings` companion to [`apply_sketch_role`] — captures the"
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_role_with_warnings
language: rust
---

# apply_sketch_role_with_warnings

`_with_warnings` companion to [`apply_sketch_role`] — captures the

## Signature

```rust
pub fn apply_sketch_role_with_warnings(
    state: &mut FootprintEditorState,
    footprint: &mut Footprint,
    id: SketchEntityId,
    role: RoleTag,
)
```

## Visibility

- `pub`

## Docstring

`_with_warnings` companion to [`apply_sketch_role`] — captures the
solver error into `state.solve_warnings` instead of propagating.

## Source
Lines 88–97 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch.md) |
| calls | [apply_sketch_role](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_role.md) |
| called_by | [set_role](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/set_role.md) |

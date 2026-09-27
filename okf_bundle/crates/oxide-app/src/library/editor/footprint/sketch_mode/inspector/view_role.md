---
okf_version: "0.2"
type: Function
title: view_role
description: v0.16.2 — Role-assignment dropdown. Visible only when a sketch
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_role
language: rust
---

# view_role

v0.16.2 — Role-assignment dropdown. Visible only when a sketch

## Signature

```rust
fn view_role(
    editor: &'a FootprintEditorState,
    text_c: Color,
    muted: Color,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

v0.16.2 — Role-assignment dropdown. Visible only when a sketch
entity is selected; pick_list value mirrors the entity's
currently-attached `*Attr` slot (or `Unassigned`). Picking a new
value emits [`FootprintEditorMsg::SketchSetRole`] which
the dispatcher routes through `apply_sketch_role_with_warnings`
(clears all attrs, sets the matching one with defaults, runs
solve + bake).

## Source
Lines 520–591 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [inspector](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [current_role_of](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/current_role_of.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view.md) |

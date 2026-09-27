---
okf_version: "0.2"
type: Function
title: view_dof
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_dof
language: rust
---

# view_dof

## Signature

```rust
fn view_dof(
    editor: &'a FootprintEditorState,
    text_c: Color,
    muted: Color,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 394–421 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [inspector](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view.md) |

---
okf_version: "0.2"
type: Function
title: apply_symbol_camera
resource: crates/oxide-app/src/library/editor/symbol/updates/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/camera/apply_symbol_camera
language: rust
---

# apply_symbol_camera

## Signature

```rust
pub(super) fn apply_symbol_camera(editor: &mut SymEditor, msg: SymbolEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 6–51 in `crates/oxide-app/src/library/editor/symbol/updates/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/library/editor/symbol/updates/camera.md) |
| calls | [symbol_bbox](/crates/oxide-app/src/library/editor/symbol/updates/mod/symbol_bbox.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |

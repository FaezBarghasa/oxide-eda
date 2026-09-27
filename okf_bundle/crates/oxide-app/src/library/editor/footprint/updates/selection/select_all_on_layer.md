---
okf_version: "0.2"
type: Function
title: select_all_on_layer
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection/select_all_on_layer
language: rust
---

# select_all_on_layer

## Signature

```rust
fn select_all_on_layer(editor: &mut crate::app::FootprintEditorState)
```

## Source
Lines 256–288 in `crates/oxide-app/src/library/editor/footprint/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/updates/selection.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/selection/apply.md) |

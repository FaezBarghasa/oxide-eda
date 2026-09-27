---
okf_version: "0.2"
type: Function
title: input_enter
description: v0.24 Track D — Enter is a no-op on state. The buffer stays alive so
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_enter
language: rust
---

# input_enter

v0.24 Track D — Enter is a no-op on state. The buffer stays alive so

## Signature

```rust
fn input_enter(_editor: &mut crate::app::FootprintEditorState)
```

## Docstring

v0.24 Track D — Enter is a no-op on state. The buffer stays alive so
the next click consumes it. The message is captured at the canvas
layer purely so the keypress doesn't fall through to a global
shortcut.

## Source
Lines 96–96 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/apply.md) |

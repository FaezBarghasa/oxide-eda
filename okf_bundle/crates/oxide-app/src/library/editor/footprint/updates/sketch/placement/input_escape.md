---
okf_version: "0.2"
type: Function
title: input_escape
description: v0.24 Track D — Esc throws away the buffer immediately; the next click
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_escape
language: rust
---

# input_escape

v0.24 Track D — Esc throws away the buffer immediately; the next click

## Signature

```rust
fn input_escape(editor: &mut crate::app::FootprintEditorState)
```

## Docstring

v0.24 Track D — Esc throws away the buffer immediately; the next click
commits at the cursor position with no override. Tool pending state is
left intact so the gesture itself isn't cancelled (use right-click /
tool Esc for that).

## Source
Lines 102–110 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/apply.md) |

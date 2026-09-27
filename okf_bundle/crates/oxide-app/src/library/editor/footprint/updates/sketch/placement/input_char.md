---
okf_version: "0.2"
type: Function
title: input_char
description: "v0.24 Track D — append `ch` to `placement_input.buffer`, minting a"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_char
language: rust
---

# input_char

v0.24 Track D — append `ch` to `placement_input.buffer`, minting a

## Signature

```rust
fn input_char(editor: &mut crate::app::FootprintEditorState, ch: char)
```

## Docstring

v0.24 Track D — append `ch` to `placement_input.buffer`, minting a
fresh entry against the active tool's matching `PlacementInputKind` if
one isn't already pinned. Drops the keypress silently when the active
tool / pending state doesn't accept numeric input.

## Source
Lines 29–77 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/apply.md) |

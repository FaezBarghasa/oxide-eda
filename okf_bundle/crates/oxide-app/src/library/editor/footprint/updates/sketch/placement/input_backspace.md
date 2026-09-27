---
okf_version: "0.2"
type: Function
title: input_backspace
description: "v0.24 Track D — pop one character; clear `placement_input` entirely"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_backspace
language: rust
---

# input_backspace

v0.24 Track D — pop one character; clear `placement_input` entirely

## Signature

```rust
fn input_backspace(editor: &mut crate::app::FootprintEditorState)
```

## Docstring

v0.24 Track D — pop one character; clear `placement_input` entirely
once the buffer empties so the next typed digit mints a fresh entry
against the (possibly different) active tool.

## Source
Lines 82–90 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/apply.md) |

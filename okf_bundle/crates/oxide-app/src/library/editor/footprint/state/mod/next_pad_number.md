---
okf_version: "0.2"
type: Function
title: next_pad_number
description: Auto-incremented pad number — picks the next integer above the
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/next_pad_number
language: rust
---

# next_pad_number

Auto-incremented pad number — picks the next integer above the

## Signature

```rust
impl FootprintEditorState { pub fn next_pad_number(&self) -> String }
```

## Visibility

- `pub`

## Docstring

Auto-incremented pad number — picks the next integer above the
current max, or "1" if none of the pads parse as integers.

## Source
Lines 363–371 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |

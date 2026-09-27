---
okf_version: "0.2"
type: Function
title: delete_pad
description: "Delete the pad at `idx`."
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/delete_pad_1
language: rust
---

# delete_pad

Delete the pad at `idx`.

## Signature

```rust
pub fn delete_pad(&mut self, idx: usize)
```

## Visibility

- `pub`

## Docstring

Delete the pad at `idx`.

## Source
Lines 480–487 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [adjust_selection_after_remove](/crates/oxide-app/src/library/editor/footprint/state/mod/adjust_selection_after_remove.md) |

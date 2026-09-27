---
okf_version: "0.2"
type: Function
title: delete_selected_closes_open_fill_picker
description: Deleting the selected graphic must close a fill picker that was
resource: crates/oxide-app/src/library/editor/symbol/updates/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/transform/delete_selected_closes_open_fill_picker
language: rust
---

# delete_selected_closes_open_fill_picker

Deleting the selected graphic must close a fill picker that was

## Signature

```rust
fn delete_selected_closes_open_fill_picker()
```

## Decorators

- `test`

## Docstring

Deleting the selected graphic must close a fill picker that was
open on it, so the transient picker state can't reopen on a
different graphic that later reuses the freed index.
[test]

## Source
Lines 105–134 in `crates/oxide-app/src/library/editor/symbol/updates/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/symbol/updates/transform.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| calls | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |

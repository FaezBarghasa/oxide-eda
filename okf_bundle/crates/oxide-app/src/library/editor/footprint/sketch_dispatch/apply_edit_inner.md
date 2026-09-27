---
okf_version: "0.2"
type: Function
title: apply_edit_inner
description: "Mutates `footprint.sketch` per the edit. Idempotent on its inputs"
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_edit_inner
language: rust
---

# apply_edit_inner

Mutates `footprint.sketch` per the edit. Idempotent on its inputs

## Signature

```rust
fn apply_edit_inner(footprint: &mut Footprint, edit: SketchEdit)
```

## Docstring

Mutates `footprint.sketch` per the edit. Idempotent on its inputs
so the test harness can inspect intermediate state.

## Source
Lines 313–351 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [apply_sketch_edit](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit.md) |

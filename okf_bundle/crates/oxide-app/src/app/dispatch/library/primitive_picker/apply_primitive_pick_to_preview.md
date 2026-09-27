---
okf_version: "0.2"
type: Function
title: apply_primitive_pick_to_preview
description: Component Preview tab — apply a freshly-picked primitive ref to
resource: crates/oxide-app/src/app/dispatch/library/primitive_picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/primitive_picker/apply_primitive_pick_to_preview
language: rust
---

# apply_primitive_pick_to_preview

Component Preview tab — apply a freshly-picked primitive ref to

## Signature

```rust
impl Oxide { fn apply_primitive_pick_to_preview(
        &mut self,
        address: EditorAddress,
        kind: PrimitiveKind,
        primitive_ref: PrimitiveRef,
    ) }
```

## Docstring

Component Preview tab — apply a freshly-picked primitive ref to
the row, resolve through the LibrarySet, save via update_row.

## Source
Lines 223–309 in `crates/oxide-app/src/app/dispatch/library/primitive_picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitive_picker](/crates/oxide-app/src/app/dispatch/library/primitive_picker.md) |
| calls | [report_read_failure](/crates/oxide-app/src/library/resolve/report_read_failure.md) |
| calls | [hash_row_content](/crates/oxide-library/src/hash/hash_row_content.md) |

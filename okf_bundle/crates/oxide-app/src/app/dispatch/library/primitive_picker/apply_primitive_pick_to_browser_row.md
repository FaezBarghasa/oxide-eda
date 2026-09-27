---
okf_version: "0.2"
type: Function
title: apply_primitive_pick_to_browser_row
description: F15 — Library Browser row binding. Same shape as
resource: crates/oxide-app/src/app/dispatch/library/primitive_picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/primitive_picker/apply_primitive_pick_to_browser_row
language: rust
---

# apply_primitive_pick_to_browser_row

F15 — Library Browser row binding. Same shape as

## Signature

```rust
impl Oxide { fn apply_primitive_pick_to_browser_row(
        &mut self,
        address: EditorAddress,
        kind: PrimitiveKind,
        primitive_ref: PrimitiveRef,
    ) }
```

## Docstring

F15 — Library Browser row binding. Same shape as
`apply_primitive_pick_to_preview` but reads/writes the row
through the cache directly because there's no Component
Preview tab open (the user picked from the inline preview /
Properties area). Updates the row, re-hashes, persists via
`adapter.update_row`, refreshes the cache.

## Source
Lines 138–219 in `crates/oxide-app/src/app/dispatch/library/primitive_picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitive_picker](/crates/oxide-app/src/app/dispatch/library/primitive_picker.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [hash_row_content](/crates/oxide-library/src/hash/hash_row_content.md) |

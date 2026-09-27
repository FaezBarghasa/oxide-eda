---
okf_version: "0.2"
type: Function
title: handle_open_primitive_picker
description: "Open the Symbol/Footprint primitive picker modal. `target`"
resource: crates/oxide-app/src/app/dispatch/library/primitive_picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/primitive_picker/handle_open_primitive_picker
language: rust
---

# handle_open_primitive_picker

Open the Symbol/Footprint primitive picker modal. `target`

## Signature

```rust
impl Oxide { pub(super) fn handle_open_primitive_picker(
        &mut self,
        kind: PrimitiveKind,
        target: PrimitivePickerTarget,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Open the Symbol/Footprint primitive picker modal. `target`
determines what happens when the user picks something.

## Source
Lines 14–26 in `crates/oxide-app/src/app/dispatch/library/primitive_picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitive_picker](/crates/oxide-app/src/app/dispatch/library/primitive_picker.md) |

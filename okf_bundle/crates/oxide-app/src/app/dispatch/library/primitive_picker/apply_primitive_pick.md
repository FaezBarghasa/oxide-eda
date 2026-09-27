---
okf_version: "0.2"
type: Function
title: apply_primitive_pick
description: "A primitive ref has been picked — apply it to the picker's"
resource: crates/oxide-app/src/app/dispatch/library/primitive_picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/primitive_picker/apply_primitive_pick
language: rust
---

# apply_primitive_pick

A primitive ref has been picked — apply it to the picker's

## Signature

```rust
impl Oxide { fn apply_primitive_pick(&mut self, primitive_ref: PrimitiveRef) -> Task<Message> }
```

## Docstring

A primitive ref has been picked — apply it to the picker's
configured target and close the modal.

## Source
Lines 82–130 in `crates/oxide-app/src/app/dispatch/library/primitive_picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitive_picker](/crates/oxide-app/src/app/dispatch/library/primitive_picker.md) |

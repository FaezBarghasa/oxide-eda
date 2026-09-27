---
okf_version: "0.2"
type: Function
title: handle_primitive_picker_msg
description: Apply a primitive picker sub-message. Most variants close the
resource: crates/oxide-app/src/app/dispatch/library/primitive_picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/primitive_picker/handle_primitive_picker_msg_1
language: rust
---

# handle_primitive_picker_msg

Apply a primitive picker sub-message. Most variants close the

## Signature

```rust
pub(super) fn handle_primitive_picker_msg(&mut self, msg: PrimitivePickerMsg) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Apply a primitive picker sub-message. Most variants close the
modal once the pick lands.

## Source
Lines 30–78 in `crates/oxide-app/src/app/dispatch/library/primitive_picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitive_picker](/crates/oxide-app/src/app/dispatch/library/primitive_picker.md) |

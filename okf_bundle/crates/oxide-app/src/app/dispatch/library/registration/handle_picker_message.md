---
okf_version: "0.2"
type: Function
title: handle_picker_message
resource: crates/oxide-app/src/app/dispatch/library/registration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/registration/handle_picker_message
language: rust
---

# handle_picker_message

## Signature

```rust
impl Oxide { pub(super) fn handle_picker_message(&mut self, msg: PickerMsg) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Source
Lines 223–259 in `crates/oxide-app/src/app/dispatch/library/registration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [registration](/crates/oxide-app/src/app/dispatch/library/registration.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |

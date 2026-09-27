---
okf_version: "0.2"
type: Function
title: handle_new_component_set_library
description: User picked a target library in the modal.
resource: crates/oxide-app/src/app/dispatch/library/new_component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/new_component/handle_new_component_set_library
language: rust
---

# handle_new_component_set_library

User picked a target library in the modal.

## Signature

```rust
impl Oxide { pub(super) fn handle_new_component_set_library(&mut self, idx: usize) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

User picked a target library in the modal.

## Source
Lines 86–92 in `crates/oxide-app/src/app/dispatch/library/new_component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [new_component](/crates/oxide-app/src/app/dispatch/library/new_component.md) |

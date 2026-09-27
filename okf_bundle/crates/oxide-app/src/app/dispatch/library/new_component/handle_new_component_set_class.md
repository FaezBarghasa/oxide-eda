---
okf_version: "0.2"
type: Function
title: handle_new_component_set_class
description: User picked a class in the modal pick_list.
resource: crates/oxide-app/src/app/dispatch/library/new_component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/new_component/handle_new_component_set_class
language: rust
---

# handle_new_component_set_class

User picked a class in the modal pick_list.

## Signature

```rust
impl Oxide { pub(super) fn handle_new_component_set_class(
        &mut self,
        class: oxide_library::ComponentClass,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

User picked a class in the modal pick_list.

## Source
Lines 95–107 in `crates/oxide-app/src/app/dispatch/library/new_component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [new_component](/crates/oxide-app/src/app/dispatch/library/new_component.md) |

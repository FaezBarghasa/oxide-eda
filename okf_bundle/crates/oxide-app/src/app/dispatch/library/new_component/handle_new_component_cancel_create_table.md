---
okf_version: "0.2"
type: Function
title: handle_new_component_cancel_create_table
description: Cancel the inline create-table form without writing anything.
resource: crates/oxide-app/src/app/dispatch/library/new_component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/new_component/handle_new_component_cancel_create_table
language: rust
---

# handle_new_component_cancel_create_table

Cancel the inline create-table form without writing anything.

## Signature

```rust
impl Oxide { pub(super) fn handle_new_component_cancel_create_table(&mut self) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Cancel the inline create-table form without writing anything.

## Source
Lines 186–191 in `crates/oxide-app/src/app/dispatch/library/new_component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [new_component](/crates/oxide-app/src/app/dispatch/library/new_component.md) |

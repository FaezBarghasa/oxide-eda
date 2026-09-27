---
okf_version: "0.2"
type: Function
title: handle_new_component_confirm_create_table
description: "Confirm — calls `create_empty_table` on the active library's"
resource: crates/oxide-app/src/app/dispatch/library/new_component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/new_component/handle_new_component_confirm_create_table_1
language: rust
---

# handle_new_component_confirm_create_table

Confirm — calls `create_empty_table` on the active library's

## Signature

```rust
pub(super) fn handle_new_component_confirm_create_table(&mut self) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Confirm — calls `create_empty_table` on the active library's
adapter, refreshes the components cache, switches the modal's
`table` selection to the freshly-minted name.

## Source
Lines 196–254 in `crates/oxide-app/src/app/dispatch/library/new_component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [new_component](/crates/oxide-app/src/app/dispatch/library/new_component.md) |

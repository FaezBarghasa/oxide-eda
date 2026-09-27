---
okf_version: "0.2"
type: Function
title: handle_new_component_set_new_table_name
description: Live-edit of the new-table name field.
resource: crates/oxide-app/src/app/dispatch/library/new_component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/new_component/handle_new_component_set_new_table_name
language: rust
---

# handle_new_component_set_new_table_name

Live-edit of the new-table name field.

## Signature

```rust
impl Oxide { pub(super) fn handle_new_component_set_new_table_name(
        &mut self,
        name: String,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Live-edit of the new-table name field.

## Source
Lines 172–183 in `crates/oxide-app/src/app/dispatch/library/new_component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [new_component](/crates/oxide-app/src/app/dispatch/library/new_component.md) |

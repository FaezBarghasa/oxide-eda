---
okf_version: "0.2"
type: Function
title: handle_new_component_submit
description: "Submit the New Component modal — creates the draft row, then"
resource: crates/oxide-app/src/app/dispatch/library/new_component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/new_component/handle_new_component_submit
language: rust
---

# handle_new_component_submit

Submit the New Component modal — creates the draft row, then

## Signature

```rust
impl Oxide { pub(super) fn handle_new_component_submit(&mut self) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Submit the New Component modal — creates the draft row, then
opens a Component Preview tab focused on the new row.

## Source
Lines 258–331 in `crates/oxide-app/src/app/dispatch/library/new_component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [new_component](/crates/oxide-app/src/app/dispatch/library/new_component.md) |
| calls | [create_component_row](/crates/oxide-app/src/library/commands/create_component_row.md) |

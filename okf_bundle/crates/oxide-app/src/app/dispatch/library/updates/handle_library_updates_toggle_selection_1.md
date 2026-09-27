---
okf_version: "0.2"
type: Function
title: handle_library_updates_toggle_selection
description: "Toggle one row's checkbox in the Library Updates modal."
resource: crates/oxide-app/src/app/dispatch/library/updates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/updates/handle_library_updates_toggle_selection_1
language: rust
---

# handle_library_updates_toggle_selection

Toggle one row's checkbox in the Library Updates modal.

## Signature

```rust
pub(super) fn handle_library_updates_toggle_selection(
        &mut self,
        symbol_uuid: uuid::Uuid,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Toggle one row's checkbox in the Library Updates modal.

## Source
Lines 16–24 in `crates/oxide-app/src/app/dispatch/library/updates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/app/dispatch/library/updates.md) |

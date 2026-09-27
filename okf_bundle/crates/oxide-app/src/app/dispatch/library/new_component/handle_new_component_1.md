---
okf_version: "0.2"
type: Function
title: handle_new_component
description: File ▸ Library ▸ New Component… — v0.13 appends a draft row
resource: crates/oxide-app/src/app/dispatch/library/new_component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/new_component/handle_new_component_1
language: rust
---

# handle_new_component

File ▸ Library ▸ New Component… — v0.13 appends a draft row

## Signature

```rust
pub(super) fn handle_new_component(&mut self) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

File ▸ Library ▸ New Component… — v0.13 appends a draft row
directly to the active library's first table and focuses it,
falling back to the legacy modal when no library / table exists.

## Source
Lines 14–74 in `crates/oxide-app/src/app/dispatch/library/new_component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [new_component](/crates/oxide-app/src/app/dispatch/library/new_component.md) |
| calls | [create_component_row](/crates/oxide-app/src/library/commands/create_component_row.md) |

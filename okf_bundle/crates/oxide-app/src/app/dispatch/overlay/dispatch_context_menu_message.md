---
okf_version: "0.2"
type: Function
title: dispatch_context_menu_message
description: Context-menu subsystem handler (canvas / project-tree / tab menus +
resource: crates/oxide-app/src/app/dispatch/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/overlay/dispatch_context_menu_message
language: rust
---

# dispatch_context_menu_message

Context-menu subsystem handler (canvas / project-tree / tab menus +

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_context_menu_message(&mut self, msg: ContextMenuMsg) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Context-menu subsystem handler (canvas / project-tree / tab menus +
submenu hover state machine), namespaced (ADR-0001 D3).

## Source
Lines 121–314 in `crates/oxide-app/src/app/dispatch/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/app/dispatch/overlay.md) |

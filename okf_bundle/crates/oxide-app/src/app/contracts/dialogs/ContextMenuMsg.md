---
okf_version: "0.2"
type: Class
title: ContextMenuMsg
description: Context-menu subsystem message family (ADR-0001 D3). Namespaced
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/ContextMenuMsg
language: rust
---

# ContextMenuMsg

Context-menu subsystem message family (ADR-0001 D3). Namespaced

## Signature

```rust
pub enum ContextMenuMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Context-menu subsystem message family (ADR-0001 D3). Namespaced
under `Message::ContextMenu` and routed to
`dispatch_context_menu_message`. Covers the canvas right-click
menu, the Projects-panel tree right-click menu, the document-tab
right-click menu, and the shared submenu hover/open state machine.
[derive(Debug, Clone)]

## Source
Lines 151–189 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |

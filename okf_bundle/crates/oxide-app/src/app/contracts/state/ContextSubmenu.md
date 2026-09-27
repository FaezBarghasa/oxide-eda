---
okf_version: "0.2"
type: Class
title: ContextSubmenu
description: Which click-to-open submenu is currently expanded inside the right-
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/ContextSubmenu
language: rust
---

# ContextSubmenu

Which click-to-open submenu is currently expanded inside the right-

## Signature

```rust
pub enum ContextSubmenu
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Which click-to-open submenu is currently expanded inside the right-
click context menu, if any. Owned by `InteractionState` and cleared
alongside `context_menu` whenever the menu closes.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 35–42 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |

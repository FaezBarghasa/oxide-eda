---
okf_version: "0.2"
type: Function
title: tab_context_menu_overlay
description: Document-tab right-click menu. Rendered before the project-tree
resource: crates/oxide-app/src/app/view/overlays/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/mod/tab_context_menu_overlay
language: rust
---

# tab_context_menu_overlay

Document-tab right-click menu. Rendered before the project-tree

## Signature

```rust
impl Oxide { pub(super) fn tab_context_menu_overlay(&self) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(super)`

## Docstring

Document-tab right-click menu. Rendered before the project-tree
menu since the two are mutually exclusive. Pushes dismiss then the
clamped menu.

## Source
Lines 462–500 in `crates/oxide-app/src/app/view/overlays/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/app/view/overlays/mod.md) |

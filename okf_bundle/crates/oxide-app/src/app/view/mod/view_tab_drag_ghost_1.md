---
okf_version: "0.2"
type: Function
title: view_tab_drag_ghost
description: Cursor-following translucent preview of a tab being dragged.
resource: crates/oxide-app/src/app/view/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/mod/view_tab_drag_ghost_1
language: rust
---

# view_tab_drag_ghost

Cursor-following translucent preview of a tab being dragged.

## Signature

```rust
fn view_tab_drag_ghost(&self, title: &str) -> Element<'_, Message>
```

## Docstring

Cursor-following translucent preview of a tab being dragged.
Shape matches the real tab bar entry — rounded container with
the title text, the ↗ undock indicator, and the × close icon —
so it reads as "the tab itself is moving". The ghost is
non-interactive; it just shows what the user is carrying.

## Source
Lines 133–163 in `crates/oxide-app/src/app/view/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/app/view/mod.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |

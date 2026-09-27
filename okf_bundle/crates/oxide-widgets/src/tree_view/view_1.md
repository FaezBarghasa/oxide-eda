---
okf_version: "0.2"
type: Function
title: view
description: Build the tree into a scrollable Element.
resource: crates/oxide-widgets/src/tree_view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/tree_view/view_1
language: rust
---

# view

Build the tree into a scrollable Element.

## Signature

```rust
pub fn view(self) -> Element<'static, TreeMsg>
```

## Visibility

- `pub`

## Docstring

Build the tree into a scrollable Element.

## Source
Lines 367–380 in `crates/oxide-widgets/src/tree_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tree_view](/crates/oxide-widgets/src/tree_view.md) |
| calls | [render_node](/crates/oxide-widgets/src/tree_view/render_node.md) |

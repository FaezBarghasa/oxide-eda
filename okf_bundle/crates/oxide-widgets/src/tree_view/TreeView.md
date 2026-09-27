---
okf_version: "0.2"
type: Class
title: TreeView
description: Composable tree view widget.
resource: crates/oxide-widgets/src/tree_view.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/tree_view/TreeView
language: rust
---

# TreeView

Composable tree view widget.

## Signature

```rust
pub struct TreeView
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Composable tree view widget.

```rust,ignore
TreeView::new(&roots, &tokens)
.selected(&path)
.view()
.map(PanelMsg::Tree)
```

## Methods

- `roots`
- `selected`
- `tokens`

## Source
Lines 346–350 in `crates/oxide-widgets/src/tree_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tree_view](/crates/oxide-widgets/src/tree_view.md) |

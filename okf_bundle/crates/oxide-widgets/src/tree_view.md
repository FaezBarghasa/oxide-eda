---
okf_version: "0.2"
type: Module
title: tree_view
description: Tree view widget — Altium-style project/component browser tree.
resource: crates/oxide-widgets/src/tree_view.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/tree_view
language: rust
---

# tree_view

Tree view widget — Altium-style project/component browser tree.

## Docstring

Tree view widget — Altium-style project/component browser tree.

COSMIC composition pattern: `TreeView` struct with builder methods.
Matches the React reference (ProjectPanel.tsx) spacing pixel-for-pixel:
- 14px indent per depth + 10px base left padding
- 6px gap between elements (gap-1.5)
- 5px vertical row padding
- 12px body font, 10px badge
- text-secondary default → text-primary on selected
- hover bg, selection bg, "(empty)" indicator

## Relationships

| Type | Target |
|------|--------|
| related | [chevron_right_handle](/crates/oxide-widgets/src/tree_view/chevron_right_handle.md) |
| related | [chevron_down_handle](/crates/oxide-widgets/src/tree_view/chevron_down_handle.md) |
| related | [TreeIcon](/crates/oxide-widgets/src/tree_view/TreeIcon.md) |
| related | [svg](/crates/oxide-widgets/src/tree_view/svg.md) |
| related | [for_path](/crates/oxide-widgets/src/tree_view/for_path.md) |
| related | [svg](/crates/oxide-widgets/src/tree_view/svg.md) |
| related | [for_path](/crates/oxide-widgets/src/tree_view/for_path.md) |
| related | [TreeNode](/crates/oxide-widgets/src/tree_view/TreeNode.md) |
| related | [leaf](/crates/oxide-widgets/src/tree_view/leaf.md) |
| related | [branch](/crates/oxide-widgets/src/tree_view/branch.md) |
| related | [with_badge](/crates/oxide-widgets/src/tree_view/with_badge.md) |
| related | [with_accent](/crates/oxide-widgets/src/tree_view/with_accent.md) |
| related | [with_open](/crates/oxide-widgets/src/tree_view/with_open.md) |
| related | [with_dirty](/crates/oxide-widgets/src/tree_view/with_dirty.md) |
| related | [with_active](/crates/oxide-widgets/src/tree_view/with_active.md) |
| related | [leaf](/crates/oxide-widgets/src/tree_view/leaf.md) |
| related | [branch](/crates/oxide-widgets/src/tree_view/branch.md) |
| related | [with_badge](/crates/oxide-widgets/src/tree_view/with_badge.md) |
| related | [with_accent](/crates/oxide-widgets/src/tree_view/with_accent.md) |
| related | [with_open](/crates/oxide-widgets/src/tree_view/with_open.md) |
| related | [with_dirty](/crates/oxide-widgets/src/tree_view/with_dirty.md) |
| related | [with_active](/crates/oxide-widgets/src/tree_view/with_active.md) |
| related | [TreeMsg](/crates/oxide-widgets/src/tree_view/TreeMsg.md) |
| related | [TreeView](/crates/oxide-widgets/src/tree_view/TreeView.md) |
| related | [new](/crates/oxide-widgets/src/tree_view/new.md) |
| related | [selected](/crates/oxide-widgets/src/tree_view/selected.md) |
| related | [view](/crates/oxide-widgets/src/tree_view/view.md) |
| related | [new](/crates/oxide-widgets/src/tree_view/new.md) |
| related | [selected](/crates/oxide-widgets/src/tree_view/selected.md) |
| related | [view](/crates/oxide-widgets/src/tree_view/view.md) |
| related | [render_node](/crates/oxide-widgets/src/tree_view/render_node.md) |
| related | [get_node](/crates/oxide-widgets/src/tree_view/get_node.md) |
| related | [toggle](/crates/oxide-widgets/src/tree_view/toggle.md) |
| related | [get_node_mut](/crates/oxide-widgets/src/tree_view/get_node_mut.md) |
| related | [iced](/_dependencies/cargo/iced.md) |

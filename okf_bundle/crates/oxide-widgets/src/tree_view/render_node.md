---
okf_version: "0.2"
type: Function
title: render_node
description: ─── Row Rendering ────────────────────────────────────────────
resource: crates/oxide-widgets/src/tree_view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/tree_view/render_node
language: rust
---

# render_node

─── Row Rendering ────────────────────────────────────────────

## Signature

```rust
fn render_node(
    mut col: Column<'static, TreeMsg>,
    node: &TreeNode,
    depth: usize,
    path: &[usize],
    selected: Option<&[usize]>,
    tokens: &ThemeTokens,
) -> Column<'static, TreeMsg>
```

## Docstring

─── Row Rendering ────────────────────────────────────────────

## Source
Lines 385–624 in `crates/oxide-widgets/src/tree_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tree_view](/crates/oxide-widgets/src/tree_view.md) |
| calls | [text_primary](/crates/oxide-widgets/src/theme_ext/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [selection_color](/crates/oxide-widgets/src/theme_ext/selection_color.md) |
| calls | [hover_color](/crates/oxide-widgets/src/theme_ext/hover_color.md) |
| calls | [chevron_down_handle](/crates/oxide-widgets/src/tree_view/chevron_down_handle.md) |
| calls | [chevron_right_handle](/crates/oxide-widgets/src/tree_view/chevron_right_handle.md) |
| calls | [accent_color](/crates/oxide-widgets/src/theme_ext/accent_color.md) |
| called_by | [view](/crates/oxide-widgets/src/tree_view/view.md) |

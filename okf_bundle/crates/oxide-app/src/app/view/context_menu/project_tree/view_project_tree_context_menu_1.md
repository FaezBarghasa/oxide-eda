---
okf_version: "0.2"
type: Function
title: view_project_tree_context_menu
description: Build the Projects-panel tree-view right-click menu. The item set
resource: crates/oxide-app/src/app/view/context_menu/project_tree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/project_tree/view_project_tree_context_menu_1
language: rust
---

# view_project_tree_context_menu

Build the Projects-panel tree-view right-click menu. The item set

## Signature

```rust
pub(in crate::app::view) fn view_project_tree_context_menu(
        &self,
        ctx: &crate::app::ProjectTreeContextMenuState,
    ) -> Element<'_, Message>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Build the Projects-panel tree-view right-click menu. The item set
is derived from the clicked node's [`TreeNodeRole`] — project root
vs library leaf vs openable leaf vs container branch — so the menu
matches what Altium shows in each context. Empty-area clicks are
filtered upstream (no menu shown), so `path` is `Some` whenever
this runs.

## Source
Lines 64–416 in `crates/oxide-app/src/app/view/context_menu/project_tree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_tree](/crates/oxide-app/src/app/view/context_menu/project_tree.md) |
| calls | [get_node](/crates/oxide-widgets/src/tree_view/get_node.md) |
| calls | [tree_node_role](/crates/oxide-app/src/app/view/context_menu/project_tree/tree_node_role.md) |
| calls | [dd_disabled](/crates/oxide-app/src/app/view/context_menu/items/dd_disabled.md) |
| calls | [dd_msg](/crates/oxide-app/src/app/view/context_menu/items/dd_msg.md) |
| calls | [ProjectTreeAction](/crates/oxide-app/src/app/contracts/state/ProjectTreeAction.md) |
| calls | [submenu_launcher](/crates/oxide-app/src/app/view/context_menu/items/submenu_launcher.md) |
| calls | [save_entry](/crates/oxide-app/src/app/view/context_menu/items/save_entry.md) |

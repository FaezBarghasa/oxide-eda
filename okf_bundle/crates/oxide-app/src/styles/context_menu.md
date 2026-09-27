---
okf_version: "0.2"
type: Function
title: context_menu
description: "Context menu / popup container (right-click menu, panel list)"
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/context_menu
language: rust
---

# context_menu

Context menu / popup container (right-click menu, panel list)

## Signature

```rust
pub fn context_menu(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Context menu / popup container (right-click menu, panel list)

## Source
Lines 194–213 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_net_color_custom_picker](/crates/oxide-app/src/app/view/modals/view_net_color_custom_picker.md) |
| called_by | [panel_list_overlay](/crates/oxide-app/src/app/view/overlays/bars/panel_list_overlay.md) |
| called_by | [view](/crates/oxide-app/src/find_replace/view.md) |

---
okf_version: "0.2"
type: Function
title: view_bar
description: Render the Active Bar (the floating toolbar strip).
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/view_bar
language: rust
---

# view_bar

Render the Active Bar (the floating toolbar strip).

## Signature

```rust
pub fn view_bar(
    current_tool: crate::app::Tool,
    draw_mode: crate::app::DrawMode,
    last_tool: &std::collections::HashMap<String, ActiveBarAction>,
    tokens: &'a ThemeTokens,
    tid: ThemeId,
    has_selection: bool,
    has_net_colors: bool,
) -> Element<'a, ActiveBarMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the Active Bar (the floating toolbar strip).

## Source
Lines 478–673 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
| calls | [action_icon](/crates/oxide-app/src/active_bar/mod/action_icon.md) |
| calls | [action_enabled](/crates/oxide-app/src/active_bar/mod/action_enabled.md) |
| called_by | [schematic_active_bar_overlay](/crates/oxide-app/src/app/view/overlays/bars/schematic_active_bar_overlay.md) |

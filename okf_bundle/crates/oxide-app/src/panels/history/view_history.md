---
okf_version: "0.2"
type: Function
title: view_history
description: Render the History panel. Delegates row rendering to
resource: crates/oxide-app/src/panels/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/history/view_history
language: rust
---

# view_history

Render the History panel. Delegates row rendering to

## Signature

```rust
pub fn view_history(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the History panel. Delegates row rendering to
[`oxide_widgets::history_pane`]; layers on the "no active
file" / "not in a git repo" / "loading" header + the working-
tree pseudo-card.

## Source
Lines 83–147 in `crates/oxide-app/src/panels/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/panels/history.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [panel_bg](/crates/oxide-widgets/src/theme_ext/panel_bg.md) |
| calls | [message_card](/crates/oxide-app/src/panels/history/message_card.md) |
| calls | [working_tree_card](/crates/oxide-app/src/panels/history/working_tree_card.md) |
| calls | [commit_card](/crates/oxide-app/src/panels/history/commit_card.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |

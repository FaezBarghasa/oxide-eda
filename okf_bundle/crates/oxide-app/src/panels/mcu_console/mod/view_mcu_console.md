---
okf_version: "0.2"
type: Function
title: view_mcu_console
resource: crates/oxide-app/src/panels/mcu_console/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:20:06Z"
concept_id: crates/oxide-app/src/panels/mcu_console/mod/view_mcu_console
language: rust
---

# view_mcu_console

## Signature

```rust
pub fn view_mcu_console(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 36–233 in `crates/oxide-app/src/panels/mcu_console/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcu_console](/crates/oxide-app/src/panels/mcu_console/mod.md) |
| calls | [panel_card](/crates/oxide-app/src/styles/panel_card.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [accent](/crates/oxide-widgets/src/theme_ext/accent.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |

---
okf_version: "0.2"
type: Function
title: view_waveform
resource: crates/oxide-app/src/panels/waveform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:19:56Z"
concept_id: crates/oxide-app/src/panels/waveform/mod/view_waveform
language: rust
---

# view_waveform

## Signature

```rust
pub fn view_waveform(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 27–166 in `crates/oxide-app/src/panels/waveform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [waveform](/crates/oxide-app/src/panels/waveform/mod.md) |
| calls | [panel_card](/crates/oxide-app/src/styles/panel_card.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [menu_item](/crates/oxide-app/src/styles/menu_item.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |

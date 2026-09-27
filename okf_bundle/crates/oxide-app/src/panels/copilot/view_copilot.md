---
okf_version: "0.2"
type: Function
title: view_copilot
resource: crates/oxide-app/src/panels/copilot.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:13:32Z"
concept_id: crates/oxide-app/src/panels/copilot/view_copilot
language: rust
---

# view_copilot

## Signature

```rust
pub fn view_copilot(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 19–103 in `crates/oxide-app/src/panels/copilot.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [copilot](/crates/oxide-app/src/panels/copilot.md) |
| calls | [panel_card](/crates/oxide-app/src/styles/panel_card.md) |
| calls | [accent](/crates/oxide-widgets/src/theme_ext/accent.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |

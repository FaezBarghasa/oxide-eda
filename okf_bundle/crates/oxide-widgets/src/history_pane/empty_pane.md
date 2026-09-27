---
okf_version: "0.2"
type: Function
title: empty_pane
resource: crates/oxide-widgets/src/history_pane.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/history_pane/empty_pane
language: rust
---

# empty_pane

## Signature

```rust
fn empty_pane(tokens: &ThemeTokens) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M`

## Source
Lines 126–147 in `crates/oxide-widgets/src/history_pane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history_pane](/crates/oxide-widgets/src/history_pane.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [panel_bg](/crates/oxide-widgets/src/theme_ext/panel_bg.md) |
| called_by | [history_pane](/crates/oxide-widgets/src/history_pane/history_pane.md) |

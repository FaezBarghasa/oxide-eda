---
okf_version: "0.2"
type: Function
title: view_erc
resource: crates/oxide-app/src/panels/status.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/status/view_erc
language: rust
---

# view_erc

## Signature

```rust
pub fn view_erc(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 35–216 in `crates/oxide-app/src/panels/status.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [status](/crates/oxide-app/src/panels/status.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [error_color](/crates/oxide-widgets/src/theme_ext/error_color.md) |
| calls | [warning_color](/crates/oxide-widgets/src/theme_ext/warning_color.md) |
| calls | [accent](/crates/oxide-widgets/src/theme_ext/accent.md) |
| calls | [selection_color](/crates/oxide-widgets/src/theme_ext/selection_color.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |

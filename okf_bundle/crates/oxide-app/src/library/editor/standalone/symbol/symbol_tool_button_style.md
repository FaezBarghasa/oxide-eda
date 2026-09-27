---
okf_version: "0.2"
type: Function
title: symbol_tool_button_style
resource: crates/oxide-app/src/library/editor/standalone/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/symbol/symbol_tool_button_style
language: rust
---

# symbol_tool_button_style

## Signature

```rust
fn symbol_tool_button_style(
    active: bool,
    border: iced::Color,
) -> impl Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style
```

## Source
Lines 220–237 in `crates/oxide-app/src/library/editor/standalone/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/library/editor/standalone/symbol.md) |
| called_by | [view_symbol_status](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_status.md) |
| called_by | [view_symbol_toolbar](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_toolbar.md) |

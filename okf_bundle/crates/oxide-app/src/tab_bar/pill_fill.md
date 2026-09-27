---
okf_version: "0.2"
type: Function
title: pill_fill
description: "Resolve the pill bg fill for the current state. Altium parity:"
resource: crates/oxide-app/src/tab_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/tab_bar/pill_fill
language: rust
---

# pill_fill

Resolve the pill bg fill for the current state. Altium parity:

## Signature

```rust
fn pill_fill(tokens: &ThemeTokens, is_active: bool, is_dragging: bool) -> iced::Color
```

## Docstring

Resolve the pill bg fill for the current state. Altium parity:
inactive tabs are dimmer than active so they read as "off" but
still show their tab body (transparent inactive made them look
like floating labels — the strip showed through). Active uses
`tokens.hover` at full alpha; inactive 0.35× the same; drag
tints with theme accent at 22 %.

## Source
Lines 148–161 in `crates/oxide-app/src/tab_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tab_bar](/crates/oxide-app/src/tab_bar.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_pdf_tab_strip](/crates/oxide-app/src/app/view/pdf_preview/mod/view_pdf_tab_strip.md) |
| called_by | [view](/crates/oxide-app/src/tab_bar/view.md) |

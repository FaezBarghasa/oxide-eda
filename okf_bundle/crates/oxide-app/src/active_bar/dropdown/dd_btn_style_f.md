---
okf_version: "0.2"
type: Function
title: dd_btn_style_f
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/dd_btn_style_f
language: rust
---

# dd_btn_style_f

## Signature

```rust
fn dd_btn_style_f(
    text_c: Color,
    hover_c: Color,
) -> impl Fn(&Theme, button::Status) -> button::Style
```

## Source
Lines 902–918 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| called_by | [net_color_entries](/crates/oxide-app/src/active_bar/dropdown/net_color_entries.md) |

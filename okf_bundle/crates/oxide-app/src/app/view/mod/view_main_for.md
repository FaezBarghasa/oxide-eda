---
okf_version: "0.2"
type: Function
title: view_main_for
resource: crates/oxide-app/src/app/view/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/mod/view_main_for
language: rust
---

# view_main_for

## Signature

```rust
impl Oxide { fn view_main_for(&self, window_id: iced::window::Id) -> Element<'_, Message> }
```

## Source
Lines 165–499 in `crates/oxide-app/src/app/view/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/app/view/mod.md) |
| calls | [wrap_plain](/crates/oxide-app/src/menu_bar/mod/wrap_plain.md) |
| calls | [chrome_separator](/crates/oxide-app/src/styles/chrome_separator.md) |
| calls | [toolbar_strip](/crates/oxide-app/src/styles/toolbar_strip.md) |

---
okf_version: "0.2"
type: Function
title: dd_item
description: "One icon + label dropdown row. Disabled rows drop their `on_press`"
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/dd_item
language: rust
---

# dd_item

One icon + label dropdown row. Disabled rows drop their `on_press`

## Signature

```rust
fn dd_item(
    icon: svg::Handle,
    label: &'static str,
    action: ActiveBarAction,
    has_selection: bool,
    has_net_colors: bool,
) -> DropdownEntry<ActiveBarMsg>
```

## Docstring

One icon + label dropdown row. Disabled rows drop their `on_press`
(the widget greys the row and ignores clicks — Altium parity).

## Source
Lines 55–75 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| calls | [action_enabled](/crates/oxide-app/src/active_bar/mod/action_enabled.md) |
| calls | [action_label](/crates/oxide-app/src/app/command/active_bar/action_label.md) |
| called_by | [render](/crates/oxide-app/src/active_bar/dropdown/render.md) |

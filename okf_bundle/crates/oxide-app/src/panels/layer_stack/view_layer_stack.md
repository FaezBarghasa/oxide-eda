---
okf_version: "0.2"
type: Function
title: view_layer_stack
resource: crates/oxide-app/src/panels/layer_stack.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:38:04Z"
concept_id: crates/oxide-app/src/panels/layer_stack/view_layer_stack
language: rust
---

# view_layer_stack

## Signature

```rust
pub fn view_layer_stack(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 11–78 in `crates/oxide-app/src/panels/layer_stack.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layer_stack](/crates/oxide-app/src/panels/layer_stack.md) |
| calls | [panel_card](/crates/oxide-app/src/styles/panel_card.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |

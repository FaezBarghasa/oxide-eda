---
okf_version: "0.2"
type: Function
title: view_parameter_manager_body
description: Altium-style Parameter Manager — a scrolling table listing every
resource: crates/oxide-app/src/app/view/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/modals/view_parameter_manager_body
language: rust
---

# view_parameter_manager_body

Altium-style Parameter Manager — a scrolling table listing every

## Signature

```rust
impl Oxide { pub(super) fn view_parameter_manager_body(&self) -> Element<'_, Message> }
```

## Visibility

- `pub(super)`

## Docstring

Altium-style Parameter Manager — a scrolling table listing every
placed symbol with columns for reference / value / footprint and
a "Parameter" column that reveals the union of custom fields
across the design. Each cell is a text_input so the user can edit
values inline. Changes route through Command::SetSymbolField so
undo/redo / dirty-flagging behaves.

## Source
Lines 335–482 in `crates/oxide-app/src/app/view/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/modals.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |

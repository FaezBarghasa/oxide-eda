---
okf_version: "0.2"
type: Function
title: view_pin_node_table
resource: crates/oxide-app/src/library/editor/sim/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:12:05Z"
concept_id: crates/oxide-app/src/library/editor/sim/mod/view_pin_node_table
language: rust
---

# view_pin_node_table

## Signature

```rust
fn view_pin_node_table(
    sim: &'a oxide_library::SimModel,
    pins: &'a [oxide_library::SymbolPin],
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 210–255 in `crates/oxide-app/src/library/editor/sim/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/editor/sim/mod.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [pin_node_row](/crates/oxide-app/src/library/editor/sim/mod/pin_node_row.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/sim/mod/view.md) |

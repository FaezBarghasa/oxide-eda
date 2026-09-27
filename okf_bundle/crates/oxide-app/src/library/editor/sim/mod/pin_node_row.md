---
okf_version: "0.2"
type: Function
title: pin_node_row
resource: crates/oxide-app/src/library/editor/sim/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:12:05Z"
concept_id: crates/oxide-app/src/library/editor/sim/mod/pin_node_row
language: rust
---

# pin_node_row

## Signature

```rust
fn pin_node_row(
    sim: &'a oxide_library::SimModel,
    pin: &'a oxide_library::SymbolPin,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 257–320 in `crates/oxide-app/src/library/editor/sim/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/editor/sim/mod.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| called_by | [view_pin_node_table](/crates/oxide-app/src/library/editor/sim/mod/view_pin_node_table.md) |

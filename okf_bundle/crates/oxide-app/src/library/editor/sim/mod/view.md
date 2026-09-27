---
okf_version: "0.2"
type: Function
title: view
resource: crates/oxide-app/src/library/editor/sim/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:12:05Z"
concept_id: crates/oxide-app/src/library/editor/sim/mod/view
language: rust
---

# view

## Signature

```rust
pub fn view(
    state: &'a ComponentPreviewState,
    tokens: &'a ThemeTokens,
    address: EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 59–208 in `crates/oxide-app/src/library/editor/sim/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/editor/sim/mod.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| calls | [SimKindPick](/crates/oxide-app/src/library/editor/sim/mod/SimKindPick.md) |
| calls | [view_pin_node_table](/crates/oxide-app/src/library/editor/sim/mod/view_pin_node_table.md) |

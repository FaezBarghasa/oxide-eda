---
okf_version: "0.2"
type: Function
title: view
description: Render the STEP attachment row in the Body 3D pane.
resource: crates/oxide-app/src/library/editor/footprint/step_attach.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/footprint/step_attach/view
language: rust
---

# view

Render the STEP attachment row in the Body 3D pane.

## Signature

```rust
pub fn view(
    fp: &'a Footprint,
    tokens: &'a ThemeTokens,
    address: EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the STEP attachment row in the Body 3D pane.

## Source
Lines 24–120 in `crates/oxide-app/src/library/editor/footprint/step_attach.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [step_attach](/crates/oxide-app/src/library/editor/footprint/step_attach.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |

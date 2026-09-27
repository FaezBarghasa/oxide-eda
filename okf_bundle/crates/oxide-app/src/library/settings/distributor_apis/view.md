---
okf_version: "0.2"
type: Function
title: view
description: Render the Distributor APIs panel. Mounts inside the existing
resource: crates/oxide-app/src/library/settings/distributor_apis.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/distributor_apis/view
language: rust
---

# view

Render the Distributor APIs panel. Mounts inside the existing

## Signature

```rust
pub fn view(
    settings: &'a DistributorSettings,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the Distributor APIs panel. Mounts inside the existing
Preferences modal under a Library section.

Phase 1 ships this panel; Phase 2 wires it into the
`crate::preferences` modal as a dedicated pref pane.

## Source
Lines 25–224 in `crates/oxide-app/src/library/settings/distributor_apis.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_apis](/crates/oxide-app/src/library/settings/distributor_apis.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [secondary_btn](/crates/oxide-app/src/library/settings/distributor_apis/secondary_btn.md) |
| calls | [primary_btn](/crates/oxide-app/src/library/settings/distributor_apis/primary_btn.md) |
| calls | [distributor_label](/crates/oxide-app/src/library/settings/distributor_apis/distributor_label.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |

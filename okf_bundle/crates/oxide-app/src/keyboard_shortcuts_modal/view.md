---
okf_version: "0.2"
type: Function
title: view
resource: crates/oxide-app/src/keyboard_shortcuts_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/keyboard_shortcuts_modal/view
language: rust
---

# view

## Signature

```rust
pub fn view(
    tokens: &'a ThemeTokens,
    theme_id: ThemeId,
    profiles: &ShortcutProfileSet,
) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 23–162 in `crates/oxide-app/src/keyboard_shortcuts_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keyboard_shortcuts_modal](/crates/oxide-app/src/keyboard_shortcuts_modal.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| calls | [thin_divider](/crates/oxide-app/src/keyboard_shortcuts_modal/thin_divider.md) |
| calls | [title_case](/crates/oxide-app/src/keyboard_shortcuts_modal/title_case.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |

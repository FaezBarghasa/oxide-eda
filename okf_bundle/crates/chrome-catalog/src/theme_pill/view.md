---
okf_version: "0.2"
type: Function
title: view
resource: crates/chrome-catalog/src/theme_pill.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:chrome-catalog"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/chrome-catalog/src/theme_pill/view
language: rust
---

# view

## Signature

```rust
pub(crate) fn view(
    theme_id: ThemeId,
    is_selected: bool,
    tokens: &ThemeTokens,
) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub(crate)`

## Source
Lines 8–43 in `crates/chrome-catalog/src/theme_pill.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme_pill](/crates/chrome-catalog/src/theme_pill.md) |
| calls | [color](/crates/chrome-catalog/src/theme/color.md) |

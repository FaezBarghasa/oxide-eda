---
okf_version: "0.2"
type: Function
title: view
resource: crates/chrome-catalog/src/section.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:chrome-catalog"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/chrome-catalog/src/section/view
language: rust
---

# view

## Signature

```rust
pub(crate) fn view(
    title: &'static str,
    tokens: &ThemeTokens,
    body: Element<'a, Message>,
) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub(crate)`

## Source
Lines 8–39 in `crates/chrome-catalog/src/section.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [section](/crates/chrome-catalog/src/section.md) |
| calls | [color](/crates/chrome-catalog/src/theme/color.md) |

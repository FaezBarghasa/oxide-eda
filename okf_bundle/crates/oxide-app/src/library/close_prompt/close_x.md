---
okf_version: "0.2"
type: Function
title: close_x
resource: crates/oxide-app/src/library/close_prompt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/close_prompt/close_x
language: rust
---

# close_x

## Signature

```rust
fn close_x(message: LibraryMessage, tokens: &ThemeTokens) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 164–190 in `crates/oxide-app/src/library/close_prompt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [close_prompt](/crates/oxide-app/src/library/close_prompt.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |

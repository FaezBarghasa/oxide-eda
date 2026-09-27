---
okf_version: "0.2"
type: Function
title: placeholder_card
resource: crates/oxide-app/src/library/editor/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/mod/placeholder_card
language: rust
---

# placeholder_card

## Signature

```rust
pub(crate) fn placeholder_card(
    title: &'a str,
    todos: &'a [&'a str],
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Decorators

- `expect(
    dead_code,
    reason = "kept for tabs still bottoming out in TODO state during the Wave-3 refactor"
)`

## Visibility

- `pub(crate)`

## Source
Lines 264–279 in `crates/oxide-app/src/library/editor/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/library/editor/mod.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |

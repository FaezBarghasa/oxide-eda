---
okf_version: "0.2"
type: Function
title: view
resource: crates/oxide-app/src/find_replace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/find_replace/view
language: rust
---

# view

## Signature

```rust
pub fn view(state: &'a FindReplaceState, tokens: &ThemeTokens) -> Element<'a, FindReplaceMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 38–223 in `crates/oxide-app/src/find_replace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [find_replace](/crates/oxide-app/src/find_replace.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [toolbar_strip](/crates/oxide-app/src/styles/toolbar_strip.md) |
| calls | [menu_item](/crates/oxide-app/src/styles/menu_item.md) |
| calls | [context_menu](/crates/oxide-app/src/styles/context_menu.md) |

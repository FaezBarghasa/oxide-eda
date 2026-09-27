---
okf_version: "0.2"
type: Function
title: theme_card
resource: crates/oxide-app/src/preferences/appearance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/preferences/appearance/theme_card
language: rust
---

# theme_card

## Signature

```rust
fn theme_card(
    id: ThemeId,
    name: &str,
    desc: &'static str,
    current: ThemeId,
) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Source
Lines 397–467 in `crates/oxide-app/src/preferences/appearance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [appearance](/crates/oxide-app/src/preferences/appearance.md) |
| called_by | [content_appearance](/crates/oxide-app/src/preferences/appearance/content_appearance.md) |

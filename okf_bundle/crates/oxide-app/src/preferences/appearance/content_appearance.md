---
okf_version: "0.2"
type: Function
title: content_appearance
resource: crates/oxide-app/src/preferences/appearance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/preferences/appearance/content_appearance
language: rust
---

# content_appearance

## Signature

```rust
pub(super) fn content_appearance(v: PrefsView<'a>) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Source
Lines 16–395 in `crates/oxide-app/src/preferences/appearance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [appearance](/crates/oxide-app/src/preferences/appearance.md) |
| calls | [theme_card](/crates/oxide-app/src/preferences/appearance/theme_card.md) |
| calls | [h_sep](/crates/oxide-app/src/preferences/widgets/h_sep.md) |
| called_by | [build_content](/crates/oxide-app/src/preferences/mod/build_content.md) |

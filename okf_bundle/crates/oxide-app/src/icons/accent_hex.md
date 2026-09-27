---
okf_version: "0.2"
type: Function
title: accent_hex
description: "Build the theme accent as a lowercase `#rrggbb` literal so the"
resource: crates/oxide-app/src/icons.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/icons/accent_hex
language: rust
---

# accent_hex

Build the theme accent as a lowercase `#rrggbb` literal so the

## Signature

```rust
fn accent_hex(theme: ThemeId) -> String
```

## Docstring

Build the theme accent as a lowercase `#rrggbb` literal so the
bytewise string replace lines up with the sentinel.

## Source
Lines 50–53 in `crates/oxide-app/src/icons.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [icons](/crates/oxide-app/src/icons.md) |
| calls | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
| called_by | [tinted_handle](/crates/oxide-app/src/icons/tinted_handle.md) |

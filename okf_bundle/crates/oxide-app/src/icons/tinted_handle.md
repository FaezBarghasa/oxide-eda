---
okf_version: "0.2"
type: Function
title: tinted_handle
description: Swap the sentinel hex for the theme accent and hand the bytes to
resource: crates/oxide-app/src/icons.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/icons/tinted_handle
language: rust
---

# tinted_handle

Swap the sentinel hex for the theme accent and hand the bytes to

## Signature

```rust
fn tinted_handle(canonical: &'static [u8], theme: ThemeId) -> svg::Handle
```

## Docstring

Swap the sentinel hex for the theme accent and hand the bytes to
iced. When the theme accent already equals the sentinel (Oxide
default) the canonical bytes go straight through — no allocation.

## Source
Lines 58–71 in `crates/oxide-app/src/icons.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [icons](/crates/oxide-app/src/icons.md) |
| calls | [accent_hex](/crates/oxide-app/src/icons/accent_hex.md) |

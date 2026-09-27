---
okf_version: "0.2"
type: Function
title: floating_panels_overlay
description: "Floating panels — one `Translate`-positioned widget per floating"
resource: crates/oxide-app/src/app/view/overlays/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/mod/floating_panels_overlay
language: rust
---

# floating_panels_overlay

Floating panels — one `Translate`-positioned widget per floating

## Signature

```rust
impl Oxide { pub(super) fn floating_panels_overlay(&self) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(super)`

## Docstring

Floating panels — one `Translate`-positioned widget per floating
panel. No clamp: panels follow Altium behaviour and may be
dragged anywhere, even past the window edge (the OS clips).

## Source
Lines 683–700 in `crates/oxide-app/src/app/view/overlays/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/app/view/overlays/mod.md) |

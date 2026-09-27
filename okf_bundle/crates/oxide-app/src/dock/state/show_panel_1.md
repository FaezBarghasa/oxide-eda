---
okf_version: "0.2"
type: Function
title: show_panel
description: "\"Open panel `kind`\" — dock it at its default region when it is"
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/show_panel_1
language: rust
---

# show_panel

"Open panel `kind`" — dock it at its default region when it is

## Signature

```rust
pub fn show_panel(&mut self, kind: PanelKind)
```

## Visibility

- `pub`

## Docstring

"Open panel `kind`" — dock it at its default region when it is
nowhere yet, otherwise bring the copy that already exists into
view. Every user-facing open gesture goes through this or
[`Self::show_panel_at`].

## Source
Lines 82–84 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |

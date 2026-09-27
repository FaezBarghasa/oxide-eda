---
okf_version: "0.2"
type: Function
title: claim_escape
description: "Esc is forwarded raw and resolved in `dispatch/escape.rs` against"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/claim_escape_1
language: rust
---

# claim_escape

Esc is forwarded raw and resolved in `dispatch/escape.rs` against

## Signature

```rust
fn claim_escape(window: iced::window::Id, event: &keyboard::Event) -> Claim
```

## Docstring

Esc is forwarded raw and resolved in `dispatch/escape.rs` against
live state, carrying the window it was typed in (#535 / #547).

Deliberately NOT routed through the keymap resolver: that advances
the multi-stroke chord buffer and consults the active profile,
neither of which Esc has ever done.

## Source
Lines 344–356 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |

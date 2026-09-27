---
okf_version: "0.2"
type: Function
title: claim_keymap
description: "Everything else routes through the active keymap profile: forward"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/claim_keymap_1
language: rust
---

# claim_keymap

Everything else routes through the active keymap profile: forward

## Signature

```rust
fn claim_keymap(window: iced::window::Id, event: &keyboard::Event) -> Claim
```

## Docstring

Everything else routes through the active keymap profile: forward
the raw stroke, resolved in `dispatch/keymap.rs` where the
multi-stroke chord buffer lives in `UiState`.

A stroke iced cannot express as a `KeyStroke` (a bare modifier
press) is ignored. Being last, `Pass` and `Swallow` are equivalent
here — `Pass` says the honest thing: nobody wanted it.

## Source
Lines 428–439 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |

---
okf_version: "0.2"
type: Function
title: claim_selection_slots
description: "Ctrl+1-8 store selection memory, Alt+1-8 recall it. These carry"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/claim_selection_slots_1
language: rust
---

# claim_selection_slots

Ctrl+1-8 store selection memory, Alt+1-8 recall it. These carry

## Signature

```rust
fn claim_selection_slots(event: &keyboard::Event) -> Claim
```

## Docstring

Ctrl+1-8 store selection memory, Alt+1-8 recall it. These carry
the digit as data the keymap profile format cannot express, so
they stay hardcoded.

The `selection_slot_from_key` guard is load-bearing (#127):
without it these arms matched EVERY Ctrl/Alt chord and swallowed
it, shadowing Ctrl+C/X/V/D before they reached the keymap
resolver below.

## Source
Lines 397–419 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
| calls | [selection_slot_from_key](/crates/oxide-app/src/app/dispatch/input/selection_slot_from_key.md) |

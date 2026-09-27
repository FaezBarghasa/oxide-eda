---
okf_version: "0.2"
type: Function
title: claim_shortcuts_sheet
description: "F1 toggles the keyboard-shortcuts sheet: open if closed, close if"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/claim_shortcuts_sheet_1
language: rust
---

# claim_shortcuts_sheet

F1 toggles the keyboard-shortcuts sheet: open if closed, close if

## Signature

```rust
fn claim_shortcuts_sheet(&self, target: InputTarget, event: &keyboard::Event) -> Claim
```

## Docstring

F1 toggles the keyboard-shortcuts sheet: open if closed, close if
open. Stays hardcoded because it depends on which modal is open —
app state, not the keymap profile.

## Source
Lines 361–387 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |

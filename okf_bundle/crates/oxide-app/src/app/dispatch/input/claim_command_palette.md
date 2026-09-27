---
okf_version: "0.2"
type: Function
title: claim_command_palette
description: Command palette. Captures most input while open so typing into
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/claim_command_palette
language: rust
---

# claim_command_palette

Command palette. Captures most input while open so typing into

## Signature

```rust
impl Oxide { fn claim_command_palette(&self, target: InputTarget, event: &keyboard::Event) -> Claim }
```

## Docstring

Command palette. Captures most input while open so typing into
the search field doesn't fire tool shortcuts (`p`, `w`, `l`, …).
Only navigation and dismiss keys leak through.

## Source
Lines 303–336 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |

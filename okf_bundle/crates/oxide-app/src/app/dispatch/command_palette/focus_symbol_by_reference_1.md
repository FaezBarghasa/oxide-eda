---
okf_version: "0.2"
type: Function
title: focus_symbol_by_reference
description: "Resolve a placed symbol's world position from the active"
resource: crates/oxide-app/src/app/dispatch/command_palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/command_palette/focus_symbol_by_reference_1
language: rust
---

# focus_symbol_by_reference

Resolve a placed symbol's world position from the active

## Signature

```rust
fn focus_symbol_by_reference(&self, reference: &str) -> Task<Message>
```

## Docstring

Resolve a placed symbol's world position from the active
engine and dispatch a `FocusAt` to centre + select it. No-op
when nothing matches (e.g. the symbol was deleted between
catalog build and Enter press).

## Source
Lines 118–141 in `crates/oxide-app/src/app/dispatch/command_palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_palette](/crates/oxide-app/src/app/dispatch/command_palette.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |

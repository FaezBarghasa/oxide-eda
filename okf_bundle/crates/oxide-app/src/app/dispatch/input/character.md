---
okf_version: "0.2"
type: Function
title: character
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/character
language: rust
---

# character

## Signature

```rust
fn character(c: &str, modifiers: keyboard::Modifiers) -> keyboard::Event
```

## Source
Lines 504–506 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
| calls | [press](/crates/oxide-app/src/app/dispatch/input/press.md) |
| called_by | [the_palette_leaks_only_its_four_navigation_keys](/crates/oxide-app/src/app/dispatch/input/the_palette_leaks_only_its_four_navigation_keys.md) |
| called_by | [the_recorder_outranks_the_palette](/crates/oxide-app/src/app/dispatch/input/the_recorder_outranks_the_palette.md) |

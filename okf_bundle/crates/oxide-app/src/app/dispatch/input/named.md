---
okf_version: "0.2"
type: Function
title: named
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/named
language: rust
---

# named

## Signature

```rust
fn named(key: keyboard::key::Named) -> keyboard::Event
```

## Source
Lines 500–502 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
| calls | [press](/crates/oxide-app/src/app/dispatch/input/press.md) |
| called_by | [the_palette_leaks_only_its_four_navigation_keys](/crates/oxide-app/src/app/dispatch/input/the_palette_leaks_only_its_four_navigation_keys.md) |

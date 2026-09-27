---
okf_version: "0.2"
type: Function
title: variant_name
description: "The variant name of an action, for the by-name comparisons above."
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar/variant_name
language: rust
---

# variant_name

The variant name of an action, for the by-name comparisons above.

## Signature

```rust
fn variant_name(action: &ActiveBarAction) -> String
```

## Docstring

The variant name of an action, for the by-name comparisons above.
`ActiveBarAction` has no discriminant accessor, and `Debug` is stable
for the unit variants this is used on.

## Source
Lines 174–181 in `crates/oxide-app/src/app/command/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/command/active_bar.md) |
| called_by | [action_label](/crates/oxide-app/src/app/command/active_bar/action_label.md) |

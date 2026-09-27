---
okf_version: "0.2"
type: Function
title: selection_slot_only_matches_digits_one_through_eight
description: "Moved here with the helper. The `None` half is a regression"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/selection_slot_only_matches_digits_one_through_eight
language: rust
---

# selection_slot_only_matches_digits_one_through_eight

Moved here with the helper. The `None` half is a regression

## Signature

```rust
fn selection_slot_only_matches_digits_one_through_eight()
```

## Decorators

- `test`

## Docstring

Moved here with the helper. The `None` half is a regression
guard for #103: the Ctrl/Alt+1-8 arms are gated on
`selection_slot_from_key(c).is_some()`, and these letters
returning `None` is exactly what lets Ctrl+C/X/V/D reach the
keymap resolver instead of being shadowed into a no-op.
[test]

## Source
Lines 965–976 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |

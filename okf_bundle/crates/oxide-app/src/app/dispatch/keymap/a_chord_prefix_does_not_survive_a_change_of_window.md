---
okf_version: "0.2"
type: Function
title: a_chord_prefix_does_not_survive_a_change_of_window
description: "The chord buffer is app-wide, so a prefix begun in one window must"
resource: crates/oxide-app/src/app/dispatch/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/keymap/a_chord_prefix_does_not_survive_a_change_of_window
language: rust
---

# a_chord_prefix_does_not_survive_a_change_of_window

The chord buffer is app-wide, so a prefix begun in one window must

## Signature

```rust
fn a_chord_prefix_does_not_survive_a_change_of_window()
```

## Decorators

- `test`

## Docstring

The chord buffer is app-wide, so a prefix begun in one window must
not be completed in another.

Tested against `begin_or_continue_chord` rather than through
`resolve_keymap_stroke`: the resolver clears the buffer again on a
definite miss, which would leave it empty either way and make the
assertion vacuous.
[test]

## Source
Lines 267–295 in `crates/oxide-app/src/app/dispatch/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/app/dispatch/keymap.md) |
| calls | [app_with_footprint_tab](/crates/oxide-app/src/app/dispatch/keymap/app_with_footprint_tab.md) |

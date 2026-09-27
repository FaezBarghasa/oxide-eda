---
okf_version: "0.2"
type: Function
title: an_open_palette_no_longer_eats_escape_in_a_window_it_does_not_paint_in
description: "The headline symptom. With the palette open, an Esc typed inside a"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/an_open_palette_no_longer_eats_escape_in_a_window_it_does_not_paint_in
language: rust
---

# an_open_palette_no_longer_eats_escape_in_a_window_it_does_not_paint_in

The headline symptom. With the palette open, an Esc typed inside a

## Signature

```rust
fn an_open_palette_no_longer_eats_escape_in_a_window_it_does_not_paint_in()
```

## Decorators

- `test`

## Docstring

The headline symptom. With the palette open, an Esc typed inside a
detached modal's own window used to close the palette — which is
not painted there — so the modal the user was looking at never saw
the key.
[test]

## Source
Lines 838–850 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
| calls | [quiet_app](/crates/oxide-app/src/app/dispatch/input/quiet_app.md) |
| calls | [open_window](/crates/oxide-app/src/app/dispatch/input/open_window.md) |

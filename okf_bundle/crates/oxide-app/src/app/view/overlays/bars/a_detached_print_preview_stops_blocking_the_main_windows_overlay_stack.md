---
okf_version: "0.2"
type: Function
title: a_detached_print_preview_stops_blocking_the_main_windows_overlay_stack
description: "#547 — the painter kept suppressing the whole overlay stack for a"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/a_detached_print_preview_stops_blocking_the_main_windows_overlay_stack
language: rust
---

# a_detached_print_preview_stops_blocking_the_main_windows_overlay_stack

#547 — the painter kept suppressing the whole overlay stack for a

## Signature

```rust
fn a_detached_print_preview_stops_blocking_the_main_windows_overlay_stack()
```

## Decorators

- `test`

## Docstring

#547 — the painter kept suppressing the whole overlay stack for a
print preview that had moved into its own OS window, while
`print_preview_overlay` had already stopped painting the in-window
card. The main window then painted NOTHING: `collect_overlays`
early-returns on this predicate, and all five pre-blocking
builders return empty. Ctrl+G, the quit confirm, Preferences and
the library modals all opened their state with nothing on screen.
[test]

## Source
Lines 772–788 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
| calls | [blank_preview](/crates/oxide-app/src/app/view/overlays/bars/blank_preview.md) |

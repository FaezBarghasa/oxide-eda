---
okf_version: "0.2"
type: Function
title: paints_overlay_stack
description: Whether this window paints the main overlay stack.
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/paints_overlay_stack
language: rust
---

# paints_overlay_stack

Whether this window paints the main overlay stack.

## Signature

```rust
impl InputTarget { pub(crate) fn paints_overlay_stack(self) -> bool }
```

## Visibility

- `pub(crate)`

## Docstring

Whether this window paints the main overlay stack.

`view_main_for` pushes `collect_overlays()` unconditionally and is
shared by the main window and every undocked tab; every other
window kind renders only its own body. So this is exactly the
question "would the user see an overlay from here" — which is
what the window-gated consumers need in phase 2 (#555).

## Source
Lines 107–109 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |

---
okf_version: "0.2"
type: Function
title: has_blocking_modal
description: True when a modal that must own the entire overlay stack is up —
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/has_blocking_modal_1
language: rust
---

# has_blocking_modal

True when a modal that must own the entire overlay stack is up —

## Signature

```rust
pub(in crate::app::view) fn has_blocking_modal(&self) -> bool
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

True when a modal that must own the entire overlay stack is up —
the error notice, the netlist-incomplete prompt, print preview, or
the custom net-colour picker. Mirrors the inline guard that
early-returns from `collect_overlays` before any tool/menu overlay
is pushed.

The print-preview term is detachment-filtered (#547). It was not
until then, and the omission cost the main window its entire
overlay stack: once the preview moved into its own OS window
`print_preview_overlay` stopped painting the in-window card, but
this predicate still reported the stack blocked, so
`collect_overlays` took its early return and every one of the five
pre-blocking builders then returned nothing. Ctrl+G grid
properties, the quit confirm, Preferences, the library modals —
all opened their state, none appeared, and nothing on screen
explained why. Detaching a modal is precisely the gesture that
stops it covering this window.

Only Print Preview needs the filter: `error_notice` and
`netlist_incomplete_prompt` have no `ModalId` and so no detached
form, and the custom net-colour picker
(`ui_state.net_color_custom`) is a different overlay from the
detachable F5 palette (`ModalId::NetColorPalette`,
`ui_state.net_color_palette_open`), which is not one of these four.

`OpenOverlays::has_blocking_modal` (`app/bootstrap/subscription.rs`)
is the Esc ladder's copy of this predicate, built from a snapshot
rather than live state. The two must agree term for term or Esc
resolves against a stack that is not on screen.

## Source
Lines 45–51 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |

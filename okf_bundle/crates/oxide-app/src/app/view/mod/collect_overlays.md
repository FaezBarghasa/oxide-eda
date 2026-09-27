---
okf_version: "0.2"
type: Function
title: collect_overlays
description: Assemble the floating overlay stack painted over the main view.
resource: crates/oxide-app/src/app/view/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/mod/collect_overlays
language: rust
---

# collect_overlays

Assemble the floating overlay stack painted over the main view.

## Signature

```rust
impl Oxide { fn collect_overlays(&self) -> Vec<Element<'_, Message>> }
```

## Docstring

Assemble the floating overlay stack painted over the main view.

Pure assembler: each overlay's guard + widget tree lives in a
dedicated `*_overlay` builder (see the `overlays` module and its
`bars` / `modals` submodules), and the order they stack in lives
in [`overlay_id::PAINT_ORDER`] — walked FORWARD here, so later
entries paint on top of earlier ones.

That same array is walked BACKWARD by the Esc ladder
(`app/bootstrap/subscription.rs`), which is the whole point of
#535: "Esc closes the topmost overlay" stops being a rule two
files must obey by hand — in opposite directions — and becomes
one list read from both ends.

[`overlay_id::visible`] applies the blocking-modal cutoff, which
is why this loop has no early return of its own.

## Source
Lines 721–748 in `crates/oxide-app/src/app/view/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/app/view/mod.md) |
| calls | [visible](/crates/oxide-app/src/app/view/overlay_id/visible.md) |
| calls | [swallows_scroll](/crates/oxide-app/src/app/view/overlay_id/swallows_scroll.md) |

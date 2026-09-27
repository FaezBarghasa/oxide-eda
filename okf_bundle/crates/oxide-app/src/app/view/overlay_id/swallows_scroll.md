---
okf_version: "0.2"
type: Function
title: swallows_scroll
description: The overlays that can actually reach the screen right now.
resource: crates/oxide-app/src/app/view/overlay_id.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlay_id/swallows_scroll
language: rust
---

# swallows_scroll

The overlays that can actually reach the screen right now.

## Signature

```rust
pub(crate) fn swallows_scroll(id: OverlayId) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

The overlays that can actually reach the screen right now.

A blocking modal owns the stack: `collect_overlays` returns right
after the pre-blocking block, so nothing later paints even when its
state is set. Both the painter and the Esc ladder must agree on that
or Esc dismisses a card the user cannot see — see the module docs.

Callers pass their own `has_blocking_modal` on purpose — the painter's
(`Oxide::has_blocking_modal`, `bars.rs`) reads live state, the Esc
ladder's (`OpenOverlays::has_blocking_modal`) reads a snapshot — but
the two must agree term for term. They did not until #547: the ladder
filtered a print preview detached into its own OS window and the
painter did not, so with the preview detached the painter suppressed
the whole stack while nothing painted in its place, and Esc resolved
against overlays that were not on screen. The divergence predated this
module; #535 part 2 wrote it down here rather than fix it, because
unifying it inside a refactor whose whole claim is behaviour-neutrality
would have smuggled a real behaviour change through that claim.
Whether this overlay must stop a mouse wheel from reaching the canvas
underneath it (#562).

The bug: scrolling past the end of a modal's content kept going and
panned/zoomed the schematic behind it. Two facts compose into that —
`scrollable` stops capturing once it hits its limit (ordinary scroll
chaining), and nothing below it captured either. `Stack::update`
walks its layers top-down and returns on `shell.is_event_captured()`
(`iced_widget-0.14.2/src/stack.rs`), so a layer that captures does
shield what is under it — there simply was not one.

Wiring `on_scroll` on the two backdrops (`wrap_modal`,
`dismiss_layer`) was necessary but not sufficient: most overlays build
neither. Preferences, for one, renders its body directly. So the
guard is applied in `collect_overlays` instead, where every overlay
passes through exactly once no matter how its builder is written.

Exhaustive on purpose: a new overlay has to say whether the wheel
stops at it.

## Source
Lines 210–270 in `crates/oxide-app/src/app/view/overlay_id.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay_id](/crates/oxide-app/src/app/view/overlay_id.md) |
| called_by | [collect_overlays](/crates/oxide-app/src/app/view/mod/collect_overlays.md) |

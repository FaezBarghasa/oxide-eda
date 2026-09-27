---
okf_version: "0.2"
type: Module
title: overlay_id
description: "Overlay identity and paint order (#535)."
resource: crates/oxide-app/src/app/view/overlay_id.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlay_id
language: rust
---

# overlay_id

Overlay identity and paint order (#535).

## Docstring

Overlay identity and paint order (#535).

One ordered list, [`PAINT_ORDER`], that both consumers derive from:

* the painter — `collect_overlays` walks it FORWARD, so later entries
stack on top of earlier ones;
* the Esc ladder — `OpenOverlays::escape_message`
(`app/bootstrap/subscription.rs`) walks it BACKWARD and takes the
first open overlay, i.e. the topmost painted one.

Before this module the two orders were maintained by hand in two
places, in opposite directions, with a doc comment asking the next
author to keep them in sync. Every ordering bug found across #514's
review rounds was that rule violated in one spot. Now there is one
list and no rule to remember: moving a variant here moves the paint
slot and the Esc rung together, because they are the same fact read
from opposite ends.

# Openness is not visibility

A blocking modal suppresses everything painted after it, and the
suppressed overlays' state stays set — Alt+F4 sets `app_quit_confirm`
with no modal check anywhere on that path, and keymap strokes are not
modal-gated either. So an overlay can be *open* and not *painted*.
[`visible`] is the one predicate that resolves that difference, and
both sides go through it; neither may walk [`PAINT_ORDER`] directly.

## Relationships

| Type | Target |
|------|--------|
| related | [OverlayId](/crates/oxide-app/src/app/view/overlay_id/OverlayId.md) |
| related | [swallows_scroll](/crates/oxide-app/src/app/view/overlay_id/swallows_scroll.md) |
| related | [visible](/crates/oxide-app/src/app/view/overlay_id/visible.md) |
| related | [extend_overlay](/crates/oxide-app/src/app/view/overlay_id/extend_overlay.md) |
| related | [extend_overlay](/crates/oxide-app/src/app/view/overlay_id/extend_overlay.md) |
| related | [paint_order_is_exhaustive_and_unique](/crates/oxide-app/src/app/view/overlay_id/paint_order_is_exhaustive_and_unique.md) |
| related | [a_blocking_modal_cuts_the_stack_down_to_the_pre_blocking_prefix](/crates/oxide-app/src/app/view/overlay_id/a_blocking_modal_cuts_the_stack_down_to_the_pre_blocking_prefix.md) |
| related | [the_pre_blocking_prefix_is_exactly_the_five_that_share_the_early_slot](/crates/oxide-app/src/app/view/overlay_id/the_pre_blocking_prefix_is_exactly_the_five_that_share_the_early_slot.md) |

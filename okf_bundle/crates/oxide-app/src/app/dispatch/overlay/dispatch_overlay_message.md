---
okf_version: "0.2"
type: Function
title: dispatch_overlay_message
resource: crates/oxide-app/src/app/dispatch/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/overlay/dispatch_overlay_message
language: rust
---

# dispatch_overlay_message

## Signature

```rust
impl Oxide { pub(super) fn dispatch_overlay_message(&mut self, message: OverlayMsg) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Source
Lines 6–62 in `crates/oxide-app/src/app/dispatch/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/app/dispatch/overlay.md) |
| calls | [write_first_run_tour_dismissed](/crates/oxide-app/src/fonts/misc/write_first_run_tour_dismissed.md) |

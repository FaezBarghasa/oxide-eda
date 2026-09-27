---
okf_version: "0.2"
type: Function
title: visible
resource: crates/oxide-app/src/app/view/overlay_id.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlay_id/visible
language: rust
---

# visible

## Signature

```rust
pub(crate) fn visible(has_blocking_modal: bool) -> &'static [OverlayId]
```

## Visibility

- `pub(crate)`

## Source
Lines 272–278 in `crates/oxide-app/src/app/view/overlay_id.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay_id](/crates/oxide-app/src/app/view/overlay_id.md) |
| called_by | [escape_message](/crates/oxide-app/src/app/bootstrap/subscription/escape_message.md) |
| called_by | [collect_overlays](/crates/oxide-app/src/app/view/mod/collect_overlays.md) |

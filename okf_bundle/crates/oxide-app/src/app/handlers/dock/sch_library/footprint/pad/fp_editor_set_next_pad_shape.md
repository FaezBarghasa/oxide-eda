---
okf_version: "0.2"
type: Function
title: fp_editor_set_next_pad_shape
description: v0.20 — Altium-parity Pad Properties / Pad Stack / Pad Features
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_shape
language: rust
---

# fp_editor_set_next_pad_shape

v0.20 — Altium-parity Pad Properties / Pad Stack / Pad Features

## Signature

```rust
impl Oxide { pub(crate) fn fp_editor_set_next_pad_shape(&mut self, shape: oxide_library::PadShape) -> bool }
```

## Visibility

- `pub(crate)`

## Docstring

v0.20 — Altium-parity Pad Properties / Pad Stack / Pad Features
form handlers. Each method mutates a slice of
`editor.state.next_pad_defaults` so the next `add_pad_at` mints
a pad with the user-selected stack / feature / testpoint
configuration. None of these are dirty-marking on their own —
they're "pre-placement defaults" — but the panel `refresh` runs
so the form re-reads the new value.

## Source
Lines 141–148 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |

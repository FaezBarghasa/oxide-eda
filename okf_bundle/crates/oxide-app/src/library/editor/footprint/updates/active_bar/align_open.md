---
okf_version: "0.2"
type: Function
title: align_open
description: "#370 — \"Align…\" dialog. Open/edit/cancel mutate only the"
resource: crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_open
language: rust
---

# align_open

#370 — "Align…" dialog. Open/edit/cancel mutate only the

## Signature

```rust
fn align_open(editor: &mut crate::app::FootprintEditorState)
```

## Docstring

#370 — "Align…" dialog. Open/edit/cancel mutate only the
transient `align_modal` state; Confirm composes the chosen
per-axis ops over the SAME `align_pads` helper the concrete
`AlignPads` rows use, under exactly one history snapshot.

## Source
Lines 270–273 in `crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/updates/active_bar.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply.md) |

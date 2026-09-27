---
okf_version: "0.2"
type: Function
title: footprint_pad_kind_label
resource: crates/oxide-app/src/app/runtime/footprint_summaries.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/footprint_summaries/footprint_pad_kind_label
language: rust
---

# footprint_pad_kind_label

## Signature

```rust
pub(super) fn footprint_pad_kind_label(
    pad: &crate::library::editor::footprint::state::EditorPad,
) -> &'static str
```

## Visibility

- `pub(super)`

## Source
Lines 1–14 in `crates/oxide-app/src/app/runtime/footprint_summaries.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries.md) |
| called_by | [build_footprint_editor_panel_ctx](/crates/oxide-app/src/app/runtime/footprint_ctx/build_footprint_editor_panel_ctx.md) |

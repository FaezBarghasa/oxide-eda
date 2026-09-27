---
okf_version: "0.2"
type: Function
title: build_sketch_entity_summary
resource: crates/oxide-app/src/app/runtime/footprint_summaries.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/footprint_summaries/build_sketch_entity_summary
language: rust
---

# build_sketch_entity_summary

## Signature

```rust
pub(super) fn build_sketch_entity_summary(
    editor: &crate::app::FootprintEditorState,
    id: oxide_sketch::id::SketchEntityId,
) -> Option<crate::panels::FootprintSketchEntitySummary>
```

## Visibility

- `pub(super)`

## Source
Lines 165–204 in `crates/oxide-app/src/app/runtime/footprint_summaries.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [build_footprint_editor_panel_ctx](/crates/oxide-app/src/app/runtime/footprint_ctx/build_footprint_editor_panel_ctx.md) |

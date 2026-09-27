---
okf_version: "0.2"
type: Function
title: select
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/select
language: rust
---

# select

## Signature

```rust
fn select(
    editor: &mut crate::app::FootprintEditorState,
    id: Option<oxide_sketch::id::SketchEntityId>,
    shift: bool,
)
```

## Source
Lines 87–123 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/apply.md) |

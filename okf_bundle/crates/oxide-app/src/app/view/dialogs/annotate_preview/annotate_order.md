---
okf_version: "0.2"
type: Function
title: annotate_order
description: "Symbol indices in the engine's annotate order (y, then x, then uuid)."
resource: crates/oxide-app/src/app/view/dialogs/annotate_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate_preview/annotate_order
language: rust
---

# annotate_order

Symbol indices in the engine's annotate order (y, then x, then uuid).

## Signature

```rust
fn annotate_order(sheet: &SchematicSheet) -> Vec<usize>
```

## Docstring

Symbol indices in the engine's annotate order (y, then x, then uuid).

## Source
Lines 53–70 in `crates/oxide-app/src/app/view/dialogs/annotate_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate_preview](/crates/oxide-app/src/app/view/dialogs/annotate_preview.md) |
| called_by | [preview_project_annotations](/crates/oxide-app/src/app/view/dialogs/annotate_preview/preview_project_annotations.md) |

---
okf_version: "0.2"
type: Function
title: preview_project_annotations
description: "The proposed `(current, new)` reference designators Annotate would hand"
resource: crates/oxide-app/src/app/view/dialogs/annotate_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate_preview/preview_project_annotations
language: rust
---

# preview_project_annotations

The proposed `(current, new)` reference designators Annotate would hand

## Signature

```rust
impl super::super::super::Oxide { pub(super) fn preview_project_annotations(&self) -> Vec<AnnotatePreviewEntry> }
```

## Visibility

- `pub(super)`

## Docstring

The proposed `(current, new)` reference designators Annotate would hand
out across the project. One row per symbol so the user sees the whole
project; rows where `current == proposed` are "no change".

## Source
Lines 143–170 in `crates/oxide-app/src/app/view/dialogs/annotate_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate_preview](/crates/oxide-app/src/app/view/dialogs/annotate_preview.md) |
| calls | [seed_counters](/crates/oxide-app/src/app/view/dialogs/annotate_preview/seed_counters.md) |
| calls | [annotate_order](/crates/oxide-app/src/app/view/dialogs/annotate_preview/annotate_order.md) |
| calls | [is_target](/crates/oxide-app/src/app/view/dialogs/annotate_preview/is_target.md) |
| calls | [prefix_of](/crates/oxide-app/src/app/view/dialogs/annotate_preview/prefix_of.md) |

---
okf_version: "0.2"
type: Function
title: seed_counters
description: "Highest number already claimed per prefix across `sheets` — the same seed"
resource: crates/oxide-app/src/app/view/dialogs/annotate_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate_preview/seed_counters
language: rust
---

# seed_counters

Highest number already claimed per prefix across `sheets` — the same seed

## Signature

```rust
fn seed_counters(sheets: &[(String, SchematicSheet)]) -> std::collections::HashMap<String, u32>
```

## Docstring

Highest number already claimed per prefix across `sheets` — the same seed
`handle_annotate`'s pass A builds, from the same sheet set.

## Source
Lines 35–50 in `crates/oxide-app/src/app/view/dialogs/annotate_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate_preview](/crates/oxide-app/src/app/view/dialogs/annotate_preview.md) |
| calls | [is_target](/crates/oxide-app/src/app/view/dialogs/annotate_preview/is_target.md) |
| calls | [prefix_of](/crates/oxide-app/src/app/view/dialogs/annotate_preview/prefix_of.md) |
| called_by | [preview_project_annotations](/crates/oxide-app/src/app/view/dialogs/annotate_preview/preview_project_annotations.md) |

---
okf_version: "0.2"
type: Function
title: prefix_of
description: "The alphabetic head of a reference — `\"R\"` for both `\"R12\"` and `\"R?\"`."
resource: crates/oxide-app/src/app/view/dialogs/annotate_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate_preview/prefix_of
language: rust
---

# prefix_of

The alphabetic head of a reference — `"R"` for both `"R12"` and `"R?"`.

## Signature

```rust
fn prefix_of(reference: &str) -> String
```

## Docstring

The alphabetic head of a reference — `"R"` for both `"R12"` and `"R?"`.

## Source
Lines 26–31 in `crates/oxide-app/src/app/view/dialogs/annotate_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate_preview](/crates/oxide-app/src/app/view/dialogs/annotate_preview.md) |
| called_by | [preview_project_annotations](/crates/oxide-app/src/app/view/dialogs/annotate_preview/preview_project_annotations.md) |
| called_by | [seed_counters](/crates/oxide-app/src/app/view/dialogs/annotate_preview/seed_counters.md) |

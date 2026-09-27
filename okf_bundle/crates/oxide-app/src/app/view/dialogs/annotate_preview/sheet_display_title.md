---
okf_version: "0.2"
type: Function
title: sheet_display_title
description: "The display title a sheet is shown under: its tab title when it is open,"
resource: crates/oxide-app/src/app/view/dialogs/annotate_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate_preview/sheet_display_title
language: rust
---

# sheet_display_title

The display title a sheet is shown under: its tab title when it is open,

## Signature

```rust
impl super::super::super::Oxide { fn sheet_display_title(&self, path: &Path) -> String }
```

## Docstring

The display title a sheet is shown under: its tab title when it is open,
its file stem otherwise.

## Source
Lines 75–88 in `crates/oxide-app/src/app/view/dialogs/annotate_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate_preview](/crates/oxide-app/src/app/view/dialogs/annotate_preview.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |

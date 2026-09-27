---
okf_version: "0.2"
type: Function
title: preview_sheets
description: "The sheets the change list spans, in the one order"
resource: crates/oxide-app/src/app/view/dialogs/annotate_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate_preview/preview_sheets
language: rust
---

# preview_sheets

The sheets the change list spans, in the one order

## Signature

```rust
impl super::super::super::Oxide { fn preview_sheets(&self) -> Vec<(String, SchematicSheet)> }
```

## Docstring

The sheets the change list spans, in the one order
[`crate::app::project_sheets::ordered_project_sheet_paths`] defines —
the same order `handle_annotate` walks, so the row order (and the
designator numbers derived from it) can't drift from what the action
assigns (#435).

The set comes from the one assembler — the same call `handle_annotate`
makes — rather than a private rule of its own. A preview that disagrees
with the action it previews is worse than no preview, and this one
used to disagree in both directions: it listed loose and other-project
tabs the action refuses to touch, and it hid the unlisted hierarchical
children the action renumbers *and writes back to disk* (#406).

This reads sheets that are not open from disk, and it runs from `view`.
See the module note in `view/dialogs/annotate/mod.rs` on why that is
tolerated rather than cached.

## Source
Lines 106–138 in `crates/oxide-app/src/app/view/dialogs/annotate_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate_preview](/crates/oxide-app/src/app/view/dialogs/annotate_preview.md) |
| calls | [assemble_active_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_active_project_sheets.md) |
| calls | [ordered_project_sheet_paths](/crates/oxide-app/src/app/project_sheets/ordered_project_sheet_paths.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |

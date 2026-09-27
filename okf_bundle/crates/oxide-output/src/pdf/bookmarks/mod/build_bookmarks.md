---
okf_version: "0.2"
type: Function
title: build_bookmarks
description: Walk the export context and gather every bookmark the user asked
resource: crates/oxide-output/src/pdf/bookmarks/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/bookmarks/mod/build_bookmarks
language: rust
---

# build_bookmarks

Walk the export context and gather every bookmark the user asked

## Signature

```rust
pub(crate) fn build_bookmarks(
    ctx: &ExportContext,
    opts: &PdfOptions,
    page_sheet_indices: &[usize],
    page_w_mm: f64,
    page_h_mm: f64,
    page_h_pt: f32,
) -> Vec<PendingBookmark>
```

## Visibility

- `pub(crate)`

## Docstring

Walk the export context and gather every bookmark the user asked
for via `PdfOptions`. Returns an empty vec when no bookmark
toggles are on — the caller should skip emitting `/Outlines`
entirely in that case.

## Source
Lines 72–272 in `crates/oxide-output/src/pdf/bookmarks/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod.md) |
| calls | [any_bookmark_toggle_on](/crates/oxide-output/src/pdf/bookmarks/mod/any_bookmark_toggle_on.md) |
| calls | [nets_group_active](/crates/oxide-output/src/pdf/bookmarks/mod/nets_group_active.md) |
| calls | [build_sheet_title](/crates/oxide-output/src/pdf/bookmarks/mod/build_sheet_title.md) |
| calls | [format_component_title](/crates/oxide-output/src/pdf/bookmarks/mod/format_component_title.md) |
| called_by | [components_toggle_emits_sheet_components_and_each_symbol](/crates/oxide-output/src/pdf/bookmarks/tests/components_toggle_emits_sheet_components_and_each_symbol.md) |
| called_by | [global_bookmarks_pulls_groups_to_top_level](/crates/oxide-output/src/pdf/bookmarks/tests/global_bookmarks_pulls_groups_to_top_level.md) |
| called_by | [multi_sheet_bookmarks_have_independent_groups](/crates/oxide-output/src/pdf/bookmarks/tests/multi_sheet_bookmarks_have_independent_groups.md) |
| called_by | [nets_group_emits_only_when_a_net_subtoggle_is_on](/crates/oxide-output/src/pdf/bookmarks/tests/nets_group_emits_only_when_a_net_subtoggle_is_on.md) |
| called_by | [no_toggles_yields_empty_bookmarks](/crates/oxide-output/src/pdf/bookmarks/tests/no_toggles_yields_empty_bookmarks.md) |
| called_by | [pin_bookmarks_skipped_when_subtoggle_off](/crates/oxide-output/src/pdf/bookmarks/tests/pin_bookmarks_skipped_when_subtoggle_off.md) |
| called_by | [variant_tag_appended_when_physical_structure_on](/crates/oxide-output/src/pdf/bookmarks/tests/variant_tag_appended_when_physical_structure_on.md) |
| called_by | [export](/crates/oxide-output/src/pdf/mod/export.md) |

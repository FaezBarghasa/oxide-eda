---
okf_version: "0.2"
type: Module
title: print_preview
description: "Print-preview modal handlers. Split from `menu/export.rs`."
resource: crates/oxide-app/src/app/handlers/menu/export/print_preview.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/print_preview
language: rust
---

# print_preview

Print-preview modal handlers. Split from `menu/export.rs`.

## Docstring

Print-preview modal handlers. Split from `menu/export.rs`.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_print_preview_requested](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_requested.md) |
| related | [handle_print_preview_select_page](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_select_page.md) |
| related | [handle_print_preview_set_colour_mode](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_colour_mode.md) |
| related | [handle_print_preview_set_page_range_all](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_page_range_all.md) |
| related | [handle_print_preview_set_page_range_current](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_page_range_current.md) |
| related | [handle_print_preview_set_page_range_specific](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_page_range_specific.md) |
| related | [handle_print_preview_set_specific_page_input](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_specific_page_input.md) |
| related | [handle_print_preview_set_fit_to_page](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_fit_to_page.md) |
| related | [handle_print_preview_set_include_title_block](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_include_title_block.md) |
| related | [handle_print_preview_export](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_export.md) |
| related | [handle_print_preview_close](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_close.md) |
| related | [handle_print_preview_rerender](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_rerender.md) |
| related | [rerasterize_print_preview](/crates/oxide-app/src/app/handlers/menu/export/print_preview/rerasterize_print_preview.md) |
| related | [handle_print_preview_zoom](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_zoom.md) |
| related | [handle_print_preview_set_tab](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_tab.md) |
| related | [handle_print_preview_pan_start](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_pan_start.md) |
| related | [handle_print_preview_pan_finished](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_pan_finished.md) |
| related | [handle_print_preview_toggle_file](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_toggle_file.md) |
| related | [handle_print_preview_select_all_files](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_select_all_files.md) |
| related | [handle_print_preview_clear_all_files](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_clear_all_files.md) |
| related | [handle_print_preview_set_variant](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_variant.md) |
| related | [handle_print_preview_requested](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_requested.md) |
| related | [handle_print_preview_select_page](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_select_page.md) |
| related | [handle_print_preview_set_colour_mode](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_colour_mode.md) |
| related | [handle_print_preview_set_page_range_all](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_page_range_all.md) |
| related | [handle_print_preview_set_page_range_current](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_page_range_current.md) |
| related | [handle_print_preview_set_page_range_specific](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_page_range_specific.md) |
| related | [handle_print_preview_set_specific_page_input](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_specific_page_input.md) |
| related | [handle_print_preview_set_fit_to_page](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_fit_to_page.md) |
| related | [handle_print_preview_set_include_title_block](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_include_title_block.md) |
| related | [handle_print_preview_export](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_export.md) |
| related | [handle_print_preview_close](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_close.md) |
| related | [handle_print_preview_rerender](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_rerender.md) |
| related | [rerasterize_print_preview](/crates/oxide-app/src/app/handlers/menu/export/print_preview/rerasterize_print_preview.md) |
| related | [handle_print_preview_zoom](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_zoom.md) |
| related | [handle_print_preview_set_tab](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_tab.md) |
| related | [handle_print_preview_pan_start](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_pan_start.md) |
| related | [handle_print_preview_pan_finished](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_pan_finished.md) |
| related | [handle_print_preview_toggle_file](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_toggle_file.md) |
| related | [handle_print_preview_select_all_files](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_select_all_files.md) |
| related | [handle_print_preview_clear_all_files](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_clear_all_files.md) |
| related | [handle_print_preview_set_variant](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_set_variant.md) |
| related | [iced](/_dependencies/cargo/iced.md) |

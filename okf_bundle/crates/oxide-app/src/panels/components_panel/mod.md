---
okf_version: "0.2"
type: Module
title: components_panel
description: "Components Panel — Stage 9 of `v0.9-snxlib-as-file-plan.md`."
resource: crates/oxide-app/src/panels/components_panel/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/mod
language: rust
---

# components_panel

Components Panel — Stage 9 of `v0.9-snxlib-as-file-plan.md`.

## Docstring

Components Panel — Stage 9 of `v0.9-snxlib-as-file-plan.md`.

The panel surfaces every mounted library through three collapsible
sections, classified by mount source:

1. **Project** — auto-mounted from the active workspace's
`Project.libraries` (existing flow, see
[`crate::library::commands::auto_mount_project_libraries`]).
2. **Installed** — session-scoped, opened via the "+ Add Library…"
button on the Installed section header. Wiped on app close.
3. **Global** — persisted across launches via
`<config_dir>/oxide/global_libraries.toml`. Loaded + mounted at
startup by [`global_prefs::load_and_mount_all`].

All three sources read from the same `LibraryState::open_libraries`
Vec — `LibraryState::mount_source_for` does the bucketing at view
time. The bucketing avoids double-rendering a library that's both
a project library and globally mounted (Project wins).

Stage 9 ships scaffold + data model + simple substring filter on
`mpn` / `manufacturer` / `internal_pn` / library name. The rich
search syntax (`mpn:LM317 lifecycle:preferred rated_power_mw>=500`,
plan §5) and the ghost-component drag-to-place are polish work for
a later stage.

## Relationships

| Type | Target |
|------|--------|
| related | [chevron](/crates/oxide-app/src/panels/components_panel/mod/chevron.md) |
| related | [view](/crates/oxide-app/src/panels/components_panel/mod/view.md) |
| related | [view_section](/crates/oxide-app/src/panels/components_panel/mod/view_section.md) |
| related | [view_library_block](/crates/oxide-app/src/panels/components_panel/mod/view_library_block.md) |
| related | [row_matches](/crates/oxide-app/src/panels/components_panel/mod/row_matches.md) |
| related | [view_row_button](/crates/oxide-app/src/panels/components_panel/mod/view_row_button.md) |
| related | [thin_sep](/crates/oxide-app/src/panels/components_panel/mod/thin_sep.md) |
| related | [fixture_row](/crates/oxide-app/src/panels/components_panel/mod/fixture_row.md) |
| related | [row_matches_empty_needle_passes_everything](/crates/oxide-app/src/panels/components_panel/mod/row_matches_empty_needle_passes_everything.md) |
| related | [row_matches_mpn_substring](/crates/oxide-app/src/panels/components_panel/mod/row_matches_mpn_substring.md) |
| related | [row_matches_manufacturer_case_insensitive](/crates/oxide-app/src/panels/components_panel/mod/row_matches_manufacturer_case_insensitive.md) |
| related | [row_matches_internal_pn](/crates/oxide-app/src/panels/components_panel/mod/row_matches_internal_pn.md) |
| related | [row_matches_library_name](/crates/oxide-app/src/panels/components_panel/mod/row_matches_library_name.md) |
| related | [row_matches_no_hit_returns_false](/crates/oxide-app/src/panels/components_panel/mod/row_matches_no_hit_returns_false.md) |
| related | [mount_source_order_is_three_sections](/crates/oxide-app/src/panels/components_panel/mod/mount_source_order_is_three_sections.md) |
| related | [mount_source_label_and_key_distinct](/crates/oxide-app/src/panels/components_panel/mod/mount_source_label_and_key_distinct.md) |
| related | [iced](/_dependencies/cargo/iced.md) |

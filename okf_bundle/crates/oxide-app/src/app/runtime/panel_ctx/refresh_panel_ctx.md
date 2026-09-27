---
okf_version: "0.2"
type: Function
title: refresh_panel_ctx
resource: crates/oxide-app/src/app/runtime/panel_ctx.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:32:07Z"
concept_id: crates/oxide-app/src/app/runtime/panel_ctx/refresh_panel_ctx
language: rust
---

# refresh_panel_ctx

## Signature

```rust
impl Oxide { pub(crate) fn refresh_panel_ctx(&mut self) }
```

## Visibility

- `pub(crate)`

## Source
Lines 87–516 in `crates/oxide-app/src/app/runtime/panel_ctx.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [panel_ctx](/crates/oxide-app/src/app/runtime/panel_ctx.md) |
| calls | [list_dir_or_report](/crates/oxide-app/src/app/dir_listing/list_dir_or_report.md) |
| calls | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
| calls | [configured_level_label](/crates/oxide-app/src/diagnostics/configured_level_label.md) |
| calls | [recent_entries](/crates/oxide-app/src/diagnostics/recent_entries.md) |
| calls | [build_symbol_editor_panel_ctx](/crates/oxide-app/src/app/runtime/symbol_ctx/build_symbol_editor_panel_ctx.md) |
| calls | [build_footprint_editor_panel_ctx](/crates/oxide-app/src/app/runtime/footprint_ctx/build_footprint_editor_panel_ctx.md) |
| calls | [build_project_tree](/crates/oxide-app/src/panels/projects/build_project_tree.md) |

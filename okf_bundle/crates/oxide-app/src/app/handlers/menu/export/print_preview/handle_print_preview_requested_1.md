---
okf_version: "0.2"
type: Function
title: handle_print_preview_requested
resource: crates/oxide-app/src/app/handlers/menu/export/print_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_requested_1
language: rust
---

# handle_print_preview_requested

## Signature

```rust
pub(crate) fn handle_print_preview_requested(&mut self) -> iced::Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 11–126 in `crates/oxide-app/src/app/handlers/menu/export/print_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [print_preview](/crates/oxide-app/src/app/handlers/menu/export/print_preview.md) |
| calls | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
| calls | [log_stitch_issues](/crates/oxide-app/src/app/handlers/menu/export/mod/log_stitch_issues.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |

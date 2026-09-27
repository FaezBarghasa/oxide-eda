---
okf_version: "0.2"
type: Function
title: diagnostic_count
description: "Count the diagnostics mentioning `marker`. The panel buffer is global, so"
resource: crates/oxide-app/src/app/handlers/menu/export/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:59Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/tests/diagnostic_count
language: rust
---

# diagnostic_count

Count the diagnostics mentioning `marker`. The panel buffer is global, so

## Signature

```rust
fn diagnostic_count(marker: &str) -> usize
```

## Docstring

Count the diagnostics mentioning `marker`. The panel buffer is global, so
every test that asserts on it uses a filename unique to itself.

## Source
Lines 546–552 in `crates/oxide-app/src/app/handlers/menu/export/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/handlers/menu/export/tests.md) |
| calls | [init_logging](/crates/oxide-app/src/diagnostics/init_logging.md) |
| calls | [recent_entries](/crates/oxide-app/src/diagnostics/recent_entries.md) |
| called_by | [pdf_export_proceeds_and_warns_once_per_user_action](/crates/oxide-app/src/app/handlers/menu/export/tests/pdf_export_proceeds_and_warns_once_per_user_action.md) |
| called_by | [rerasterizing_the_preview_does_not_flood_the_messages_panel](/crates/oxide-app/src/app/handlers/menu/export/tests/rerasterizing_the_preview_does_not_flood_the_messages_panel.md) |

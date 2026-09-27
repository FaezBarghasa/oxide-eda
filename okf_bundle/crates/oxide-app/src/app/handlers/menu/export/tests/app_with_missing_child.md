---
okf_version: "0.2"
type: Function
title: app_with_missing_child
description: "A project whose root references `missing` — a child that is neither open"
resource: crates/oxide-app/src/app/handlers/menu/export/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:59Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/tests/app_with_missing_child
language: rust
---

# app_with_missing_child

A project whose root references `missing` — a child that is neither open

## Signature

```rust
fn app_with_missing_child(missing: &str) -> Oxide
```

## Docstring

A project whose root references `missing` — a child that is neither open
nor on disk. The one case that is a *genuine* `MissingChild`.

## Source
Lines 556–567 in `crates/oxide-app/src/app/handlers/menu/export/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/handlers/menu/export/tests.md) |
| calls | [app_workspace](/crates/oxide-app/src/app/handlers/menu/export/tests/app_workspace.md) |
| calls | [open_with](/crates/oxide-app/src/app/handlers/menu/export/tests/open_with.md) |
| calls | [sheet_with_net](/crates/oxide-app/src/app/handlers/menu/export/tests/sheet_with_net.md) |
| called_by | [cancel_on_the_incomplete_prompt_writes_nothing](/crates/oxide-app/src/app/handlers/menu/export/tests/cancel_on_the_incomplete_prompt_writes_nothing.md) |
| called_by | [export_anyway_writes_a_partial_netlist_with_an_incomplete_header](/crates/oxide-app/src/app/handlers/menu/export/tests/export_anyway_writes_a_partial_netlist_with_an_incomplete_header.md) |
| called_by | [export_anyway_writes_from_the_prompt_snapshot_not_a_fresh_re_derivation](/crates/oxide-app/src/app/handlers/menu/export/tests/export_anyway_writes_from_the_prompt_snapshot_not_a_fresh_re_derivation.md) |
| called_by | [netlist_export_refuses_to_write_an_incomplete_netlist](/crates/oxide-app/src/app/handlers/menu/export/tests/netlist_export_refuses_to_write_an_incomplete_netlist.md) |
| called_by | [pdf_export_proceeds_and_warns_once_per_user_action](/crates/oxide-app/src/app/handlers/menu/export/tests/pdf_export_proceeds_and_warns_once_per_user_action.md) |
| called_by | [rerasterizing_the_preview_does_not_flood_the_messages_panel](/crates/oxide-app/src/app/handlers/menu/export/tests/rerasterizing_the_preview_does_not_flood_the_messages_panel.md) |

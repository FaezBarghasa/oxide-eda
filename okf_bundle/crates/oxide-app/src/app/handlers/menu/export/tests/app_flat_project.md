---
okf_version: "0.2"
type: Function
title: app_flat_project
description: "A flat two-page project: both pages are listed, neither references the"
resource: crates/oxide-app/src/app/handlers/menu/export/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:59Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/tests/app_flat_project
language: rust
---

# app_flat_project

A flat two-page project: both pages are listed, neither references the

## Signature

```rust
fn app_flat_project() -> Oxide
```

## Docstring

A flat two-page project: both pages are listed, neither references the
other as a child sheet. `project_navigation::add` appends to `data.sheets`
with no requirement that anything reference the sheet, so this is routine,
not pathological.

## Source
Lines 782–798 in `crates/oxide-app/src/app/handlers/menu/export/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/handlers/menu/export/tests.md) |
| calls | [app_workspace](/crates/oxide-app/src/app/handlers/menu/export/tests/app_workspace.md) |
| calls | [open_with](/crates/oxide-app/src/app/handlers/menu/export/tests/open_with.md) |
| calls | [sheet_with_net](/crates/oxide-app/src/app/handlers/menu/export/tests/sheet_with_net.md) |
| called_by | [a_flat_projects_second_page_no_longer_vanishes_from_the_netlist](/crates/oxide-app/src/app/handlers/menu/export/tests/a_flat_projects_second_page_no_longer_vanishes_from_the_netlist.md) |

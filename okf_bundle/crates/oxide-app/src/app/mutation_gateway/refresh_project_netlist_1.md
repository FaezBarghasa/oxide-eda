---
okf_version: "0.2"
type: Function
title: refresh_project_netlist
description: Re-derive the cached project netlist off the shared sheet view (rooted at
resource: crates/oxide-app/src/app/mutation_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/mutation_gateway/refresh_project_netlist_1
language: rust
---

# refresh_project_netlist

Re-derive the cached project netlist off the shared sheet view (rooted at

## Signature

```rust
pub(crate) fn refresh_project_netlist(&mut self)
```

## Visibility

- `pub(crate)`

## Docstring

Re-derive the cached project netlist off the shared sheet view (rooted at
the active sheet) and surface any stitch issues in the Messages panel.
A cheap no-op while the cache is still valid.

## Source
Lines 247–323 in `crates/oxide-app/src/app/mutation_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mutation_gateway](/crates/oxide-app/src/app/mutation_gateway.md) |
| calls | [assemble_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_project_sheets.md) |
| calls | [project_graph](/crates/oxide-app/src/app/project_sheets/project_graph.md) |
| calls | [sheet_key](/crates/oxide-app/src/app/project_sheets/sheet_key.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
| calls | [stitch_issue_message](/crates/oxide-app/src/app/project_sheets/stitch_issue_message.md) |
| calls | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |

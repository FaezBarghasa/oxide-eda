---
okf_version: "0.2"
type: Function
title: handle_run_erc
resource: crates/oxide-app/src/app/handlers/erc/erc_run.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/erc/erc_run/handle_run_erc_1
language: rust
---

# handle_run_erc

## Signature

```rust
pub(crate) fn handle_run_erc(&mut self) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 8–161 in `crates/oxide-app/src/app/handlers/erc/erc_run.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc_run](/crates/oxide-app/src/app/handlers/erc/erc_run.md) |
| calls | [assemble_active_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_active_project_sheets.md) |
| calls | [project_graph](/crates/oxide-app/src/app/project_sheets/project_graph.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
| calls | [stitch_issue_message](/crates/oxide-app/src/app/project_sheets/stitch_issue_message.md) |
| calls | [sheet_key](/crates/oxide-app/src/app/project_sheets/sheet_key.md) |
| calls | [run_with_project_and_dsl](/crates/oxide-erc/src/lib/run_with_project_and_dsl.md) |
| calls | [run_with_project](/crates/oxide-erc/src/lib/run_with_project.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |

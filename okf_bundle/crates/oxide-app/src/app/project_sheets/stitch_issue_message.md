---
okf_version: "0.2"
type: Function
title: stitch_issue_message
description: "A one-line, user-facing message for a cross-sheet stitch issue (ADR-0002 D7,"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/stitch_issue_message
language: rust
---

# stitch_issue_message

A one-line, user-facing message for a cross-sheet stitch issue (ADR-0002 D7,

## Signature

```rust
pub(crate) fn stitch_issue_message(issue: &oxide_net::StitchIssue) -> String
```

## Visibility

- `pub(crate)`

## Docstring

A one-line, user-facing message for a cross-sheet stitch issue (ADR-0002 D7,
part 3) — shown in the Messages panel alongside other diagnostics.

Lives here rather than in one consumer because *every* caller of
[`oxide_net::build_project_netlist`] must surface its issues: the netlist
is always produced, so a dropped `MissingChild` means an exported netlist
that is quietly missing a whole subtree.

## Source
Lines 546–586 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| called_by | [handle_run_erc](/crates/oxide-app/src/app/handlers/erc/erc_run/handle_run_erc.md) |
| called_by | [refresh_project_netlist](/crates/oxide-app/src/app/mutation_gateway/refresh_project_netlist.md) |

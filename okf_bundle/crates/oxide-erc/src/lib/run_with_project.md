---
okf_version: "0.2"
type: Function
title: run_with_project
description: Run ERC for a schematic in the context of a whole project. Cross-sheet
resource: crates/oxide-erc/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc/src/lib/run_with_project
language: rust
---

# run_with_project

Run ERC for a schematic in the context of a whole project. Cross-sheet

## Signature

```rust
pub fn run_with_project(
    snapshot: &SchematicSheet,
    resolved: &std::collections::HashMap<String, oxide_net::SheetKey>,
    sheets: &std::collections::HashMap<oxide_net::SheetKey, SchematicSheet>,
) -> Vec<Violation>
```

## Visibility

- `pub`

## Docstring

Run ERC for a schematic in the context of a whole project. Cross-sheet
rules consult `resolved` — THIS sheet's own `ChildSheet.filename ->
SheetKey` submap — resolved through the shared `sheets` table. Pass an
empty `resolved` map for top-only runs.

`resolved` must be the caller's per-parent submap for `snapshot`, not a
single project-wide map: keying it that way is what stops two parents in
different directories that reference a child by the same filename string
from resolving to the wrong file (#466), and is what keeps this in step
with `oxide_net::build_project_netlist`, which takes the same shape.

## Source
Lines 158–168 in `crates/oxide-erc/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc/src/lib.md) |
| calls | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
| called_by | [handle_run_erc](/crates/oxide-app/src/app/handlers/erc/erc_run/handle_run_erc.md) |

---
okf_version: "0.2"
type: Function
title: app_with_a_child_only_on_disk
description: "The export fixture, on purpose: the whole point is that both paths"
resource: crates/oxide-app/src/app/mutation_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/mutation_gateway/app_with_a_child_only_on_disk
language: rust
---

# app_with_a_child_only_on_disk

The export fixture, on purpose: the whole point is that both paths

## Signature

```rust
fn app_with_a_child_only_on_disk() -> (Oxide, std::path::PathBuf)
```

## Docstring

The export fixture, on purpose: the whole point is that both paths
answer "what sheets does this project consist of" the same way.

## Source
Lines 371–393 in `crates/oxide-app/src/app/mutation_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mutation_gateway](/crates/oxide-app/src/app/mutation_gateway.md) |
| calls | [sheet_with_net](/crates/oxide-app/src/app/handlers/menu/export/tests/sheet_with_net.md) |
| calls | [app_workspace](/crates/oxide-app/src/app/handlers/menu/export/tests/app_workspace.md) |
| calls | [open_with](/crates/oxide-app/src/app/handlers/menu/export/tests/open_with.md) |
| called_by | [erc_checks_a_child_that_is_only_on_disk](/crates/oxide-app/src/app/mutation_gateway/erc_checks_a_child_that_is_only_on_disk.md) |
| called_by | [the_cached_netlist_stitches_a_child_that_is_only_on_disk](/crates/oxide-app/src/app/mutation_gateway/the_cached_netlist_stitches_a_child_that_is_only_on_disk.md) |

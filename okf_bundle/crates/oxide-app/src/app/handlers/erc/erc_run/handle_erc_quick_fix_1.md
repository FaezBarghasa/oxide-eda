---
okf_version: "0.2"
type: Function
title: handle_erc_quick_fix
description: Quick Fix dispatch from the Messages-panel chip.
resource: crates/oxide-app/src/app/handlers/erc/erc_run.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/erc/erc_run/handle_erc_quick_fix_1
language: rust
---

# handle_erc_quick_fix

Quick Fix dispatch from the Messages-panel chip.

## Signature

```rust
pub(crate) fn handle_erc_quick_fix(&mut self, index: usize) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Quick Fix dispatch from the Messages-panel chip.
`UnusedPin` places a `NoConnect` at the violation's world
coords and re-runs ERC so the row disappears immediately;
every other rule falls back to the row-click "zoom + select"
path, which is exactly the affordance the user wants 90% of
the time even without a mutating fix.

## Source
Lines 287–339 in `crates/oxide-app/src/app/handlers/erc/erc_run.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc_run](/crates/oxide-app/src/app/handlers/erc/erc_run.md) |

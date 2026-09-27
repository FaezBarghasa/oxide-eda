---
okf_version: "0.2"
type: Function
title: app_with_dirty_symbol
description: "Build a minimal app carrying one dirty, open symbol-library"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/app_with_dirty_symbol
language: rust
---

# app_with_dirty_symbol

Build a minimal app carrying one dirty, open symbol-library

## Signature

```rust
fn app_with_dirty_symbol(path: &std::path::Path) -> Oxide
```

## Docstring

Build a minimal app carrying one dirty, open symbol-library
editor keyed at `path` — mirrors the state right after
`Add New ▸ Symbol Library` followed by an edit (the exact repro
for the "Export Failed — SymbolLibrary5.snxsym" close bug).

## Source
Lines 378–388 in `crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [close_project](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.md) |
| called_by | [save_all_writes_a_dirty_symbol_library_instead_of_failing](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/save_all_writes_a_dirty_symbol_library_instead_of_failing.md) |

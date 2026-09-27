---
okf_version: "0.2"
type: Class
title: TabKind
description: "Per-tab role marker. Schematic / Pcb retain the path on `TabInfo`"
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/TabKind
language: rust
---

# TabKind

Per-tab role marker. Schematic / Pcb retain the path on `TabInfo`

## Signature

```rust
pub enum TabKind
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Per-tab role marker. Schematic / Pcb retain the path on `TabInfo`
for the existing `engines` HashMap and dirty-paths machinery;
`ComponentEditor` carries its own `(library_path, table, row_id)`
payload that the dispatcher uses to look the editor state up out
of `LibraryState.editors`. The synthetic `TabInfo.path` for
ComponentEditor tabs is `<library_path>/<table>.tsv#<row_id>`
(table-fragment notation, see
`LibraryState::component_tab_path`) so undock / "is this tab
already undocked?" / per-tab visibility continue to use a single
PathBuf identity. No `.tsv#<row>` file is ever written to disk —
the fragment is purely a tab-identity salt that points back at
the row inside the table file. Per `v0.9-refactor-2-plan.md` §2.2
the legacy `.snxprt`-per-component file format is fully retired.
[derive(Debug, Clone)]

## Source
Lines 32–51 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |

---
okf_version: "0.2"
type: Function
title: an_out_of_tree_namesake_cannot_evict_the_root
description: "#536 — the root must survive a `SheetKey` collision."
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/an_out_of_tree_namesake_cannot_evict_the_root
language: rust
---

# an_out_of_tree_namesake_cannot_evict_the_root

#536 — the root must survive a `SheetKey` collision.

## Signature

```rust
fn an_out_of_tree_namesake_cannot_evict_the_root()
```

## Decorators

- `test`

## Docstring

#536 — the root must survive a `SheetKey` collision.

Reproducible on EVERY platform, not just Windows as the issue
first read. `path_key` folds case only under `cfg!(windows)`, but
`sheet_key` relativizes against `base` and falls back to the bare
`file_name()` when a path is not under it. A sheet living outside
the project directory therefore keys as `top.snxsch` and collides
with the project's own `/proj/top.snxsch`, case-folding or not.

Sorted first-wins then handed the slot to the outsider — `/other`
sorts before `/proj` — and both netlist callers, finding no root
key, bailed without a word.
[test]

## Source
Lines 1001–1014 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| calls | [sheet](/crates/oxide-app/src/app/project_sheets/sheet.md) |
| calls | [project_graph](/crates/oxide-app/src/app/project_sheets/project_graph.md) |

---
okf_version: "0.2"
type: Function
title: child_sheet_refs
description: "`sheet path → the `filename` strings it references as child sheets`,"
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/child_sheet_refs_1
language: rust
---

# child_sheet_refs

`sheet path → the `filename` strings it references as child sheets`,

## Signature

```rust
fn child_sheet_refs(&self) -> std::collections::HashMap<PathBuf, Vec<String>>
```

## Docstring

`sheet path → the `filename` strings it references as child sheets`,
over every loaded engine. Input to the hierarchy walk in
[`active_document_project`](Self::active_document_project).

## Source
Lines 705–718 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |

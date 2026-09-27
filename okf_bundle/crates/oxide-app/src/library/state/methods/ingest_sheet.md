---
okf_version: "0.2"
type: Function
title: ingest_sheet
description: "Replace the Where-Used entries for one `(project, sheet)` with"
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/ingest_sheet
language: rust
---

# ingest_sheet

Replace the Where-Used entries for one `(project, sheet)` with

## Signature

```rust
impl LibraryState { pub fn ingest_sheet(&mut self, project: &Path, sheet: &Path, refs: &[(Uuid, String)]) }
```

## Visibility

- `pub`

## Docstring

Replace the Where-Used entries for one `(project, sheet)` with
`refs` — `(row_id, instance_id)` tuples. The index keys by
`RowId` directly; revisions and per-instance version pins are
not part of the DBLib model.

## Source
Lines 282–288 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |

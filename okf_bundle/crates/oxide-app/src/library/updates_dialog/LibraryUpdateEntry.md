---
okf_version: "0.2"
type: Class
title: LibraryUpdateEntry
description: "One row in the modal — drift between a placed Symbol's pinned"
resource: crates/oxide-app/src/library/updates_dialog.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/updates_dialog/LibraryUpdateEntry
language: rust
---

# LibraryUpdateEntry

One row in the modal — drift between a placed Symbol's pinned

## Signature

```rust
pub struct LibraryUpdateEntry
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

One row in the modal — drift between a placed Symbol's pinned
version and its source row's current version.
[derive(Debug, Clone)]

## Methods

- `symbol_uuid`
- `ref_des`
- `library_id`
- `library_name`
- `row_id`
- `library_path`
- `current_version`
- `latest_version`
- `bump_kind`
- `selected`

## Source
Lines 117–146 in `crates/oxide-app/src/library/updates_dialog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates_dialog](/crates/oxide-app/src/library/updates_dialog.md) |

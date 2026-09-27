---
okf_version: "0.2"
type: Function
title: drop_project
description: "Drop every entry for `project` (called on project close)."
resource: crates/oxide-library/src/where_used.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/where_used/drop_project
language: rust
---

# drop_project

Drop every entry for `project` (called on project close).

## Signature

```rust
impl WhereUsedIndex { pub fn drop_project(&mut self, project: &Path) }
```

## Visibility

- `pub`

## Docstring

Drop every entry for `project` (called on project close).

No-op if the project is not currently indexed.

## Source
Lines 115–117 in `crates/oxide-library/src/where_used.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [where_used](/crates/oxide-library/src/where_used.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |

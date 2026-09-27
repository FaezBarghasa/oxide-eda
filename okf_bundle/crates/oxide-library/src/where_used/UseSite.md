---
okf_version: "0.2"
type: Class
title: UseSite
description: One occurrence of a row on a schematic sheet.
resource: crates/oxide-library/src/where_used.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/where_used/UseSite
language: rust
---

# UseSite

One occurrence of a row on a schematic sheet.

## Signature

```rust
pub struct UseSite
```

## Decorators

- `derive(Clone, Debug, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

One occurrence of a row on a schematic sheet.

Per the v0.9-refactor-2 model, rows have no per-row version chain —
schematic instances reference their library row by `row_id` only. The
historical change log lives in `git log` (LocalGit) or the audit table
(Database) and is surfaced separately.
[derive(Clone, Debug, PartialEq, Eq)]

## Methods

- `project_path`
- `sheet_path`
- `instance_id`

## Source
Lines 35–43 in `crates/oxide-library/src/where_used.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [where_used](/crates/oxide-library/src/where_used.md) |

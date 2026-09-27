---
okf_version: "0.2"
type: Class
title: TableConfig
description: "Optional `[[tables]]` override block — Altium DBLib parity."
resource: crates/oxide-library/src/manifest.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/manifest/TableConfig
language: rust
---

# TableConfig

Optional `[[tables]]` override block — Altium DBLib parity.

## Signature

```rust
pub struct TableConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Optional `[[tables]]` override block — Altium DBLib parity.

Per `v0.9-refactor-2-plan.md` §3:
- The default is class → `<class>s.tsv` (mechanical pluralisation).
- An explicit override lets multiple classes share one table
(e.g. resistors + capacitors both rolled into `Discrete_Passives.tsv`),
or rename the file for non-English / domain-specific layouts.

`name` is the filename stem (no extension); the file lives at
`tables/<name>.tsv`.
[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

## Methods

- `name`
- `classes`

## Source
Lines 132–138 in `crates/oxide-library/src/manifest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manifest](/crates/oxide-library/src/manifest.md) |

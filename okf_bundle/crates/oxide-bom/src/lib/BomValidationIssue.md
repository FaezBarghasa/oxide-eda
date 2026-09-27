---
okf_version: "0.2"
type: Class
title: BomValidationIssue
description: Single validation finding produced by BOM rules.
resource: crates/oxide-bom/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-bom"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bom/src/lib/BomValidationIssue
language: rust
---

# BomValidationIssue

Single validation finding produced by BOM rules.

## Signature

```rust
pub struct BomValidationIssue
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Single validation finding produced by BOM rules.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `rule`
- `severity`
- `message`
- `references`

## Source
Lines 128–133 in `crates/oxide-bom/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-bom/src/lib.md) |

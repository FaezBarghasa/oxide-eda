---
okf_version: "0.2"
type: Class
title: ResolvedRef
description: Resolved target commit and version metadata.
resource: crates/oxide-library/src/dependency/resolver.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:39Z"
concept_id: crates/oxide-library/src/dependency/resolver/ResolvedRef
language: rust
---

# ResolvedRef

Resolved target commit and version metadata.

## Signature

```rust
pub struct ResolvedRef
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Resolved target commit and version metadata.
[derive(Debug, Clone, PartialEq, Eq)]

## Methods

- `commit_oid`
- `tree_oid`
- `resolved_version`

## Source
Lines 12–16 in `crates/oxide-library/src/dependency/resolver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resolver](/crates/oxide-library/src/dependency/resolver.md) |

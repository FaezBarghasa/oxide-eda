---
okf_version: "0.2"
type: Function
title: resolve_rev
description: Resolve a direct commit hash / revision string.
resource: crates/oxide-library/src/dependency/resolver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:39Z"
concept_id: crates/oxide-library/src/dependency/resolver/resolve_rev
language: rust
---

# resolve_rev

Resolve a direct commit hash / revision string.

## Signature

```rust
impl GitResolver { fn resolve_rev(
        &self,
        repo: &git2::Repository,
        name: &str,
        rev_str: &str,
    ) -> Result<ResolvedRef, DependencyError> }
```

## Docstring

Resolve a direct commit hash / revision string.

## Source
Lines 110–136 in `crates/oxide-library/src/dependency/resolver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resolver](/crates/oxide-library/src/dependency/resolver.md) |

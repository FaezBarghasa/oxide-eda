---
okf_version: "0.2"
type: Function
title: resolve_reference
description: "Resolve a dependency's requested Git reference to a concrete commit & tree OID."
resource: crates/oxide-library/src/dependency/resolver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:39Z"
concept_id: crates/oxide-library/src/dependency/resolver/resolve_reference
language: rust
---

# resolve_reference

Resolve a dependency's requested Git reference to a concrete commit & tree OID.

## Signature

```rust
impl GitResolver { pub fn resolve_reference(
        &self,
        repo: &git2::Repository,
        dep: &ProjectDependency,
    ) -> Result<ResolvedRef, DependencyError> }
```

## Visibility

- `pub`

## Docstring

Resolve a dependency's requested Git reference to a concrete commit & tree OID.

## Source
Lines 28–39 in `crates/oxide-library/src/dependency/resolver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resolver](/crates/oxide-library/src/dependency/resolver.md) |

---
okf_version: "0.2"
type: Function
title: resolve_tag
description: Resolve an exact tag name.
resource: crates/oxide-library/src/dependency/resolver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:39Z"
concept_id: crates/oxide-library/src/dependency/resolver/resolve_tag
language: rust
---

# resolve_tag

Resolve an exact tag name.

## Signature

```rust
impl GitResolver { fn resolve_tag(
        &self,
        repo: &git2::Repository,
        name: &str,
        tag_name: &str,
    ) -> Result<ResolvedRef, DependencyError> }
```

## Docstring

Resolve an exact tag name.

## Source
Lines 42–74 in `crates/oxide-library/src/dependency/resolver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resolver](/crates/oxide-library/src/dependency/resolver.md) |

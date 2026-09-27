---
okf_version: "0.2"
type: Function
title: resolve_branch
description: Resolve a branch head.
resource: crates/oxide-library/src/dependency/resolver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:39Z"
concept_id: crates/oxide-library/src/dependency/resolver/resolve_branch_1
language: rust
---

# resolve_branch

Resolve a branch head.

## Signature

```rust
fn resolve_branch(
        &self,
        repo: &git2::Repository,
        name: &str,
        branch_name: &str,
    ) -> Result<ResolvedRef, DependencyError>
```

## Docstring

Resolve a branch head.

## Source
Lines 77–107 in `crates/oxide-library/src/dependency/resolver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resolver](/crates/oxide-library/src/dependency/resolver.md) |

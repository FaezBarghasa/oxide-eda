---
okf_version: "0.2"
type: Function
title: resolve_semver
description: Resolve semantic version requirements against available Git tags.
resource: crates/oxide-library/src/dependency/resolver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:39Z"
concept_id: crates/oxide-library/src/dependency/resolver/resolve_semver_1
language: rust
---

# resolve_semver

Resolve semantic version requirements against available Git tags.

## Signature

```rust
fn resolve_semver(
        &self,
        repo: &git2::Repository,
        name: &str,
        req_str: &str,
    ) -> Result<ResolvedRef, DependencyError>
```

## Docstring

Resolve semantic version requirements against available Git tags.

## Source
Lines 139–182 in `crates/oxide-library/src/dependency/resolver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resolver](/crates/oxide-library/src/dependency/resolver.md) |

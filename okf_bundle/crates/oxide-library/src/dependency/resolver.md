---
okf_version: "0.2"
type: Module
title: resolver
description: Git2 resolver for EDA project dependencies.
resource: crates/oxide-library/src/dependency/resolver.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:39Z"
concept_id: crates/oxide-library/src/dependency/resolver
language: rust
---

# resolver

Git2 resolver for EDA project dependencies.

## Docstring

Git2 resolver for EDA project dependencies.

Evaluates Git tags against semantic version ranges, branches, and commits.

## Relationships

| Type | Target |
|------|--------|
| related | [ResolvedRef](/crates/oxide-library/src/dependency/resolver/ResolvedRef.md) |
| related | [GitResolver](/crates/oxide-library/src/dependency/resolver/GitResolver.md) |
| related | [new](/crates/oxide-library/src/dependency/resolver/new.md) |
| related | [resolve_reference](/crates/oxide-library/src/dependency/resolver/resolve_reference.md) |
| related | [resolve_tag](/crates/oxide-library/src/dependency/resolver/resolve_tag.md) |
| related | [resolve_branch](/crates/oxide-library/src/dependency/resolver/resolve_branch.md) |
| related | [resolve_rev](/crates/oxide-library/src/dependency/resolver/resolve_rev.md) |
| related | [resolve_semver](/crates/oxide-library/src/dependency/resolver/resolve_semver.md) |
| related | [new](/crates/oxide-library/src/dependency/resolver/new.md) |
| related | [resolve_reference](/crates/oxide-library/src/dependency/resolver/resolve_reference.md) |
| related | [resolve_tag](/crates/oxide-library/src/dependency/resolver/resolve_tag.md) |
| related | [resolve_branch](/crates/oxide-library/src/dependency/resolver/resolve_branch.md) |
| related | [resolve_rev](/crates/oxide-library/src/dependency/resolver/resolve_rev.md) |
| related | [resolve_semver](/crates/oxide-library/src/dependency/resolver/resolve_semver.md) |
| related | [semver](/_dependencies/cargo/semver.md) |

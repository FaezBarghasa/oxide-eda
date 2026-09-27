---
okf_version: "0.2"
type: Module
title: adapters
description: Storage adapter implementations behind feature flags.
resource: crates/oxide-library/src/adapters/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/mod
language: rust
---

# adapters

Storage adapter implementations behind feature flags.

## Docstring

Storage adapter implementations behind feature flags.

Each flavour ships as its own module gated on a Cargo feature so
consumers only pull in the deps they actually use.

- `local-git` → [`local_git::LocalGitAdapter`] backed by a `*.snxlib/` dir
plus an embedded libgit2 repo.
- `database` → [`database::DatabaseAdapter`] HTTP client speaking to
`oxide-library-server`.
- [`library_set::LibrarySet`] composes any number of `LibraryAdapter`
trait objects into a single resolver for cross-library
[`crate::primitive::PrimitiveRef`] lookup.

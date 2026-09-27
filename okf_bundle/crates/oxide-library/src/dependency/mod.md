---
okf_version: "0.2"
type: Module
title: dependency
description: Git2-based version and dependency manager for Oxide EDA projects.
resource: crates/oxide-library/src/dependency/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:41Z"
concept_id: crates/oxide-library/src/dependency/mod
language: rust
---

# dependency

Git2-based version and dependency manager for Oxide EDA projects.

## Docstring

Git2-based version and dependency manager for Oxide EDA projects.

Manages versioned dependencies (`.snxlib` libraries, `.snxfpt` footprints,
and `.snxsym` components) declared in `.snxprj` with deterministic locking in `project.lock`.

## Relationships

| Type | Target |
|------|--------|
| related | [DependencyError](/crates/oxide-library/src/dependency/mod/DependencyError.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |

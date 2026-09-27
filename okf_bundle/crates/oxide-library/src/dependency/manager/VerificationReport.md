---
okf_version: "0.2"
type: Class
title: VerificationReport
description: Verification status report across all dependencies.
resource: crates/oxide-library/src/dependency/manager.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:52Z"
concept_id: crates/oxide-library/src/dependency/manager/VerificationReport
language: rust
---

# VerificationReport

Verification status report across all dependencies.

## Signature

```rust
pub struct VerificationReport
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Verification status report across all dependencies.
[derive(Debug, Clone, Default)]

## Methods

- `valid_dependencies`
- `missing_dependencies`
- `corrupted_dependencies`

## Source
Lines 18–22 in `crates/oxide-library/src/dependency/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-library/src/dependency/manager.md) |

---
okf_version: "0.2"
type: Function
title: verify_integrity
description: "Verify local disk state against `project.lock`."
resource: crates/oxide-library/src/dependency/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:52Z"
concept_id: crates/oxide-library/src/dependency/manager/verify_integrity_1
language: rust
---

# verify_integrity

Verify local disk state against `project.lock`.

## Signature

```rust
pub fn verify_integrity(
        &self,
        project_root: &Path,
        lockfile: &ProjectLockfile,
    ) -> Result<VerificationReport, DependencyError>
```

## Visibility

- `pub`

## Docstring

Verify local disk state against `project.lock`.

## Source
Lines 151–173 in `crates/oxide-library/src/dependency/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-library/src/dependency/manager.md) |

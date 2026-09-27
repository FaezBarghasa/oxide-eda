---
okf_version: "0.2"
type: Function
title: verify_integrity
description: Verify that an installed dependency on disk matches the exact tree OID in the lockfile.
resource: crates/oxide-library/src/dependency/installer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:18Z"
concept_id: crates/oxide-library/src/dependency/installer/verify_integrity_1
language: rust
---

# verify_integrity

Verify that an installed dependency on disk matches the exact tree OID in the lockfile.

## Signature

```rust
pub fn verify_integrity(
        &self,
        project_root: &Path,
        locked: &LockedDependency,
    ) -> Result<bool, DependencyError>
```

## Visibility

- `pub`

## Docstring

Verify that an installed dependency on disk matches the exact tree OID in the lockfile.

## Source
Lines 78–109 in `crates/oxide-library/src/dependency/installer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [installer](/crates/oxide-library/src/dependency/installer.md) |

---
okf_version: "0.2"
type: Function
title: validate_file_path
description: "Reject paths whose extension isn't `.snxlib`. We refuse rather"
resource: crates/oxide-library/src/adapters/local_git/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/mod/validate_file_path
language: rust
---

# validate_file_path

Reject paths whose extension isn't `.snxlib`. We refuse rather

## Signature

```rust
impl LocalGitAdapter { fn validate_file_path(p: &Path) -> Result<(), LibraryError> }
```

## Docstring

Reject paths whose extension isn't `.snxlib`. We refuse rather
than silently rewriting; the file picker should be filtering,
but defensively block anyway so a misnamed path doesn't end up
committed to history.

## Source
Lines 315–328 in `crates/oxide-library/src/adapters/local_git/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git](/crates/oxide-library/src/adapters/local_git/mod.md) |

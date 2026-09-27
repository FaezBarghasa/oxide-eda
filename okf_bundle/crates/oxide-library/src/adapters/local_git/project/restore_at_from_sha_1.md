---
okf_version: "0.2"
type: Function
title: restore_at_from_sha
description: "String-SHA-keyed alternative to [`restore_at`]. Parses the"
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/restore_at_from_sha_1
language: rust
---

# restore_at_from_sha

String-SHA-keyed alternative to [`restore_at`]. Parses the

## Signature

```rust
pub fn restore_at_from_sha(&self, rel_path: &Path, sha: &str) -> Result<(), LibraryError>
```

## Visibility

- `pub`

## Docstring

String-SHA-keyed alternative to [`restore_at`]. Parses the
argument as a hex commit OID, then forwards. Convenient for
callers (notably `oxide-app`) that don't depend on `git2`
directly and just have the short/full SHA string from the
History panel widget.

## Source
Lines 312–316 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |

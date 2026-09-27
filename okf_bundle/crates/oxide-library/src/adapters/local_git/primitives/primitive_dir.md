---
okf_version: "0.2"
type: Function
title: primitive_dir
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/primitive_dir
language: rust
---

# primitive_dir

## Signature

```rust
impl LocalGitAdapter { fn primitive_dir(&self, kind: PrimitiveKind) -> PathBuf }
```

## Source
Lines 7–9 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
| calls | [primitive_subdir](/crates/oxide-library/src/adapters/local_git/helpers/primitive_subdir.md) |

---
okf_version: "0.2"
type: Function
title: library_classes
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/library_classes
language: rust
---

# library_classes

## Signature

```rust
impl LocalGitAdapter { fn library_classes(&self) -> Vec<crate::library_file::ClassEntry> }
```

## Source
Lines 116–118 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
| calls | [classes_or_report](/crates/oxide-library/src/adapters/local_git/helpers/classes_or_report.md) |

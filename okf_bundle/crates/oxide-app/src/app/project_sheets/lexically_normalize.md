---
okf_version: "0.2"
type: Function
title: lexically_normalize
description: "Lexically normalize `path` — resolve `.` and `..` components without"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/lexically_normalize
language: rust
---

# lexically_normalize

Lexically normalize `path` — resolve `.` and `..` components without

## Signature

```rust
fn lexically_normalize(path: &Path) -> PathBuf
```

## Docstring

Lexically normalize `path` — resolve `.` and `..` components without
touching the filesystem (unlike `std::fs::canonicalize`, which needs the
path to exist). A `..` with nothing left to pop is kept as a literal `..`
component, so a reference that escapes its root still fails the caller's
`starts_with(root)` check instead of silently collapsing onto the root.

## Source
Lines 522–537 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| called_by | [resolve_child_reference](/crates/oxide-app/src/app/project_sheets/resolve_child_reference.md) |

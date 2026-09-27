---
okf_version: "0.2"
type: Function
title: collect_project_library_paths
resource: crates/oxide-app/src/app/runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/mod/collect_project_library_paths_1
language: rust
---

# collect_project_library_paths

## Signature

```rust
pub(crate) fn collect_project_library_paths(&self) -> Vec<std::path::PathBuf>
```

## Decorators

- `expect(
        dead_code,
        reason = "reserved for callers outside the Components Panel that need a flat library-path slice"
    )`

## Visibility

- `pub(crate)`

## Source
Lines 18–29 in `crates/oxide-app/src/app/runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [runtime](/crates/oxide-app/src/app/runtime/mod.md) |

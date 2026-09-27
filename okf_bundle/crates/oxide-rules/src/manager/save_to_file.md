---
okf_version: "0.2"
type: Function
title: save_to_file
description: "Save constraints to a `rules.toml` file on disk."
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/save_to_file
language: rust
---

# save_to_file

Save constraints to a `rules.toml` file on disk.

## Signature

```rust
impl ConstraintManager { pub fn save_to_file(
        &self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> }
```

## Type Parameters

- `P: AsRef<std::path::Path`

## Visibility

- `pub`

## Docstring

Save constraints to a `rules.toml` file on disk.

## Source
Lines 564–571 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |

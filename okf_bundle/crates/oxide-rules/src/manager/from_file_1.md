---
okf_version: "0.2"
type: Function
title: from_file
description: "Load constraints from a `rules.toml` file on disk."
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/from_file_1
language: rust
---

# from_file

Load constraints from a `rules.toml` file on disk.

## Signature

```rust
pub fn from_file(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
```

## Type Parameters

- `P: AsRef<std::path::Path`

## Visibility

- `pub`

## Docstring

Load constraints from a `rules.toml` file on disk.

## Source
Lines 555–561 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |

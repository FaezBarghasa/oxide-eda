---
okf_version: "0.2"
type: Function
title: root_path
description: "For local-git, the `.snxlib/` directory; for DB, `None`."
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/root_path
language: rust
---

# root_path

For local-git, the `.snxlib/` directory; for DB, `None`.

## Signature

```rust
fn root_path(&self) -> Option<PathBuf>
```

## Docstring

For local-git, the `.snxlib/` directory; for DB, `None`.

## Source
Lines 462–464 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |

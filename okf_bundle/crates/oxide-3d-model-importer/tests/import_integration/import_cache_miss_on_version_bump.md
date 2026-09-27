---
okf_version: "0.2"
type: Function
title: import_cache_miss_on_version_bump
description: "[test]"
resource: crates/oxide-3d-model-importer/tests/import_integration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-3d-model-importer/tests/import_integration/import_cache_miss_on_version_bump
language: rust
---

# import_cache_miss_on_version_bump

[test]

## Signature

```rust
fn import_cache_miss_on_version_bump()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 187–211 in `crates/oxide-3d-model-importer/tests/import_integration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [import_integration](/crates/oxide-3d-model-importer/tests/import_integration.md) |
| calls | [write_tier0_wrl](/crates/oxide-3d-model-importer/tests/import_integration/write_tier0_wrl.md) |
| calls | [import_model](/crates/oxide-3d-model-importer/src/lib/import_model.md) |

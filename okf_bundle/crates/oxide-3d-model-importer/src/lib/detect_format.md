---
okf_version: "0.2"
type: Function
title: detect_format
resource: crates/oxide-3d-model-importer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/lib/detect_format
language: rust
---

# detect_format

## Signature

```rust
fn detect_format(path: &Path) -> Result<SourceFormat, ModelImportError>
```

## Source
Lines 184–198 in `crates/oxide-3d-model-importer/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-3d-model-importer/src/lib.md) |
| called_by | [import_model](/crates/oxide-3d-model-importer/src/lib/import_model.md) |

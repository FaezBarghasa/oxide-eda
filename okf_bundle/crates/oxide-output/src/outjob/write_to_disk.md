---
okf_version: "0.2"
type: Function
title: write_to_disk
description: Write all generated files to a destination directory.
resource: crates/oxide-output/src/outjob.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:32:32Z"
concept_id: crates/oxide-output/src/outjob/write_to_disk
language: rust
---

# write_to_disk

Write all generated files to a destination directory.

## Signature

```rust
impl ReleasePackage { pub fn write_to_disk(&self, out_dir: &Path) -> Result<(), std::io::Error> }
```

## Visibility

- `pub`

## Docstring

Write all generated files to a destination directory.

## Source
Lines 78–88 in `crates/oxide-output/src/outjob.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [outjob](/crates/oxide-output/src/outjob.md) |

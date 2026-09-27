---
okf_version: "0.2"
type: Function
title: add_binary_file
resource: crates/oxide-output/src/outjob.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:32:32Z"
concept_id: crates/oxide-output/src/outjob/add_binary_file
language: rust
---

# add_binary_file

## Signature

```rust
impl ReleasePackage { pub fn add_binary_file(&mut self, filename: impl Into<String>, content: Vec<u8>) }
```

## Visibility

- `pub`

## Source
Lines 73–75 in `crates/oxide-output/src/outjob.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [outjob](/crates/oxide-output/src/outjob.md) |

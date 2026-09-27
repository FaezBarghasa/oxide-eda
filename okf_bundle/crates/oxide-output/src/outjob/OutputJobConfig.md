---
okf_version: "0.2"
type: Class
title: OutputJobConfig
description: Output Job definition describing all release artifacts to generate.
resource: crates/oxide-output/src/outjob.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:32:32Z"
concept_id: crates/oxide-output/src/outjob/OutputJobConfig
language: rust
---

# OutputJobConfig

Output Job definition describing all release artifacts to generate.

## Signature

```rust
pub struct OutputJobConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Output Job definition describing all release artifacts to generate.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `generate_gerber`
- `generate_drills`
- `generate_pick_and_place`
- `generate_bom`
- `generate_pdf`
- `target_folder`

## Source
Lines 37–45 in `crates/oxide-output/src/outjob.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [outjob](/crates/oxide-output/src/outjob.md) |

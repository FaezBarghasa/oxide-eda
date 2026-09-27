---
okf_version: "0.2"
type: Function
title: run
description: Execute an output job and assemble the complete release package.
resource: crates/oxide-output/src/outjob.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:32:32Z"
concept_id: crates/oxide-output/src/outjob/run_1
language: rust
---

# run

Execute an output job and assemble the complete release package.

## Signature

```rust
pub fn run(
        config: &OutputJobConfig,
        ctx: &ExportContext,
        board: &PcbBoard,
    ) -> Result<ReleasePackage, OutJobError>
```

## Visibility

- `pub`

## Docstring

Execute an output job and assemble the complete release package.

## Source
Lines 95–151 in `crates/oxide-output/src/outjob.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [outjob](/crates/oxide-output/src/outjob.md) |

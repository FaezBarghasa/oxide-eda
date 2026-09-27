---
okf_version: "0.2"
type: Function
title: annotate_with_seed
description: "Cross-sheet annotation variant. Like `Command::AnnotateAll` but uses"
resource: crates/oxide-engine/src/annotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/annotation/annotate_with_seed_1
language: rust
---

# annotate_with_seed

Cross-sheet annotation variant. Like `Command::AnnotateAll` but uses

## Signature

```rust
pub fn annotate_with_seed(
        &mut self,
        mode: AnnotateMode,
        next_by_prefix: &mut std::collections::HashMap<String, u32>,
    ) -> Result<bool, EngineError>
```

## Visibility

- `pub`

## Docstring

Cross-sheet annotation variant. Like `Command::AnnotateAll` but uses
(and updates) an externally-owned per-prefix counter, so every
sheet in a project can share one global numbering pass. Returns
whether anything changed.

## Source
Lines 12–18 in `crates/oxide-engine/src/annotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotation](/crates/oxide-engine/src/annotation.md) |

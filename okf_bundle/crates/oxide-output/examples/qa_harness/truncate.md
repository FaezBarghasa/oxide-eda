---
okf_version: "0.2"
type: Function
title: truncate
resource: crates/oxide-output/examples/qa_harness.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/examples/qa_harness/truncate
language: rust
---

# truncate

## Signature

```rust
fn truncate(s: &str, n: usize) -> String
```

## Source
Lines 271–279 in `crates/oxide-output/examples/qa_harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qa_harness](/crates/oxide-output/examples/qa_harness.md) |
| called_by | [parse](/crates/oxide-altium-importer/src/cfb/parse.md) |
| called_by | [add_constraint_solves_geometry](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/add_constraint_solves_geometry.md) |
| called_by | [write](/crates/oxide-library/src/library_file/codec/write.md) |
| called_by | [retain_frontier](/crates/oxide-widgets/src/passive_calculator/solver/retain_frontier.md) |
| called_by | [solve](/crates/oxide-widgets/src/passive_calculator/solver/solve.md) |

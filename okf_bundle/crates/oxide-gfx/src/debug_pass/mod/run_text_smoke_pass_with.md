---
okf_version: "0.2"
type: Function
title: run_text_smoke_pass_with
resource: crates/oxide-gfx/src/debug_pass/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:35:56Z"
concept_id: crates/oxide-gfx/src/debug_pass/mod/run_text_smoke_pass_with
language: rust
---

# run_text_smoke_pass_with

## Signature

```rust
fn run_text_smoke_pass_with(scale_px_per_mm: f32, texts: &[TextItem]) -> Result<u32, String>
```

## Source
Lines 608–700 in `crates/oxide-gfx/src/debug_pass/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [debug_pass](/crates/oxide-gfx/src/debug_pass/mod.md) |
| called_by | [run_text_smoke_pass](/crates/oxide-gfx/src/debug_pass/mod/run_text_smoke_pass.md) |
| called_by | [text_smoke_pass_clips_fully_outside_viewport](/crates/oxide-gfx/src/debug_pass/tests/text_smoke_pass_clips_fully_outside_viewport.md) |
| called_by | [text_smoke_pass_handles_dense_overlap_cluster](/crates/oxide-gfx/src/debug_pass/tests/text_smoke_pass_handles_dense_overlap_cluster.md) |
| called_by | [text_smoke_pass_handles_scale_rotation_and_empty_content](/crates/oxide-gfx/src/debug_pass/tests/text_smoke_pass_handles_scale_rotation_and_empty_content.md) |

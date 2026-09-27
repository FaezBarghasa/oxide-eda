---
okf_version: "0.2"
type: Function
title: markup_runs
resource: crates/oxide-output/src/svg/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/text/markup_runs
language: rust
---

# markup_runs

## Signature

```rust
fn markup_runs(input: &str) -> Vec<MarkupRun>
```

## Source
Lines 146–203 in `crates/oxide-output/src/svg/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-output/src/svg/text.md) |
| calls | [parse_oxide_markup](/crates/oxide-types/src/markup/parse_oxide_markup.md) |
| called_by | [draw_text_outline](/crates/oxide-output/src/svg/text/draw_text_outline.md) |

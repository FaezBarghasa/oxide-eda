---
okf_version: "0.2"
type: Function
title: taking_size_mm_as_em_renders_short_by_the_ratio
description: "Treating `size_mm` as the em size — what the GPU pipeline did — is"
resource: crates/oxide-gfx/src/primitive/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/text/taking_size_mm_as_em_renders_short_by_the_ratio
language: rust
---

# taking_size_mm_as_em_renders_short_by_the_ratio

Treating `size_mm` as the em size — what the GPU pipeline did — is

## Signature

```rust
fn taking_size_mm_as_em_renders_short_by_the_ratio()
```

## Decorators

- `test`

## Docstring

Treating `size_mm` as the em size — what the GPU pipeline did — is
short by exactly `MM_PER_EM`, so a 10 pt import stopped measuring
50 mils.
[test]

## Source
Lines 106–111 in `crates/oxide-gfx/src/primitive/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/primitive/text.md) |
| calls | [text_px](/crates/oxide-gfx/src/primitive/text/text_px.md) |

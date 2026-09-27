---
okf_version: "0.2"
type: Function
title: register
description: "Register a font, returning its Ref. Same font registered twice returns"
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/register_1
language: rust
---

# register

Register a font, returning its Ref. Same font registered twice returns

## Signature

```rust
pub fn register(&mut self, font: PdfFont, base_ref: i32) -> Ref
```

## Visibility

- `pub`

## Docstring

Register a font, returning its Ref. Same font registered twice returns
the same Ref.

MD-35: `base_ref` is the first Ref id this catalog is allowed to
hand out — callers reserve a contiguous range starting there.
The previous hard-coded base of `100` collided with the page-tree
allocator once the sheet count exceeded ~97.

## Source
Lines 245–252 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |

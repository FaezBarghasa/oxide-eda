---
okf_version: "0.2"
type: Function
title: bga_row_letter
description: "IPC-7351 BGA letter sequence. With `skip_letters` true, the alphabet"
resource: crates/oxide-sketch/src/array.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/array/bga_row_letter
language: rust
---

# bga_row_letter

IPC-7351 BGA letter sequence. With `skip_letters` true, the alphabet

## Signature

```rust
pub fn bga_row_letter(row_index: u32, skip_letters: bool, start_row: char) -> String
```

## Visibility

- `pub`

## Docstring

IPC-7351 BGA letter sequence. With `skip_letters` true, the alphabet
is `ABCDEFGHJKLMNPRTUVWY` (I/O/Q/S/X/Z skipped). Excel-style
extension produces `AA, AB, …` after the single-letter range.

## Source
Lines 140–164 in `crates/oxide-sketch/src/array.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [array](/crates/oxide-sketch/src/array.md) |
| called_by | [derive_pad_number_2d](/crates/oxide-bake/src/array/numbering/derive_pad_number_2d.md) |

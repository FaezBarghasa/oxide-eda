---
okf_version: "0.2"
type: Function
title: extract_pinout_from_text
description: Pure-text variant — split out for unit-testing without a real PDF.
resource: crates/oxide-library/src/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/ai_stub/extract_pinout_from_text
language: rust
---

# extract_pinout_from_text

Pure-text variant — split out for unit-testing without a real PDF.

## Signature

```rust
pub fn extract_pinout_from_text(text: &str) -> PinoutGuess
```

## Visibility

- `pub`

## Docstring

Pure-text variant — split out for unit-testing without a real PDF.

## Source
Lines 60–76 in `crates/oxide-library/src/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/src/ai_stub.md) |
| calls | [parse_pin_row](/crates/oxide-library/src/ai_stub/parse_pin_row.md) |
| calls | [score_confidence](/crates/oxide-library/src/ai_stub/score_confidence.md) |
| called_by | [dedupes_repeated_pin_numbers](/crates/oxide-library/src/ai_stub/dedupes_repeated_pin_numbers.md) |
| called_by | [empty_text_yields_zero_confidence](/crates/oxide-library/src/ai_stub/empty_text_yields_zero_confidence.md) |
| called_by | [extract_pinout](/crates/oxide-library/src/ai_stub/extract_pinout.md) |
| called_by | [garbage_text_yields_low_confidence](/crates/oxide-library/src/ai_stub/garbage_text_yields_low_confidence.md) |
| called_by | [parses_lm317_style_table_from_plain_text](/crates/oxide-library/src/ai_stub/parses_lm317_style_table_from_plain_text.md) |

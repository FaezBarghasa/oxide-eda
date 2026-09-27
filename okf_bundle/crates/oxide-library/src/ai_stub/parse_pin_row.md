---
okf_version: "0.2"
type: Function
title: parse_pin_row
description: Match a single pin-table row.
resource: crates/oxide-library/src/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/ai_stub/parse_pin_row
language: rust
---

# parse_pin_row

Match a single pin-table row.

## Signature

```rust
fn parse_pin_row(line: &str) -> Option<PinGuess>
```

## Docstring

Match a single pin-table row.

Acceptable shapes (whitespace-tolerant):
* `1 ADJ Input` — number, name, type/description
* `1   ADJ   Adjustment terminal`
* `A1 VCC Power supply`

## Source
Lines 84–99 in `crates/oxide-library/src/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/src/ai_stub.md) |
| calls | [pin_row_regex](/crates/oxide-library/src/ai_stub/pin_row_regex.md) |
| calls | [classify_pin_kind](/crates/oxide-library/src/ai_stub/classify_pin_kind.md) |
| called_by | [extract_pinout_from_text](/crates/oxide-library/src/ai_stub/extract_pinout_from_text.md) |

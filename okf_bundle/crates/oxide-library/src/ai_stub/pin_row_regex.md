---
okf_version: "0.2"
type: Function
title: pin_row_regex
description: Lazily compiled regex matching a pin-table row.
resource: crates/oxide-library/src/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/ai_stub/pin_row_regex
language: rust
---

# pin_row_regex

Lazily compiled regex matching a pin-table row.

## Signature

```rust
fn pin_row_regex() -> &'static Regex
```

## Docstring

Lazily compiled regex matching a pin-table row.

Capture groups:
1. pin number (digits, optional letter-prefix like `A1`)
2. pin name (letters, digits, `+`, `-`, `_`, `/`)
3. remainder of the line (description / type)

## Source
Lines 215–221 in `crates/oxide-library/src/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/src/ai_stub.md) |
| called_by | [parse_pin_row](/crates/oxide-library/src/ai_stub/parse_pin_row.md) |

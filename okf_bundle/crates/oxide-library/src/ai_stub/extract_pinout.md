---
okf_version: "0.2"
type: Function
title: extract_pinout
description: "Heuristic pinout extractor. **No LLM**, **no network**."
resource: crates/oxide-library/src/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/ai_stub/extract_pinout
language: rust
---

# extract_pinout

Heuristic pinout extractor. **No LLM**, **no network**.

## Signature

```rust
pub fn extract_pinout(pdf_bytes: &[u8]) -> PinoutGuess
```

## Visibility

- `pub`

## Docstring

Heuristic pinout extractor. **No LLM**, **no network**.

Returns an empty pin list with `confidence < 0.3` when:
* the PDF parser fails to extract any text;
* the extracted text contains no recognisable pin-table rows.

## Source
Lines 51–57 in `crates/oxide-library/src/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/src/ai_stub.md) |
| calls | [extract_pinout_from_text](/crates/oxide-library/src/ai_stub/extract_pinout_from_text.md) |
| called_by | [extract_pinout_handles_corrupt_bytes_gracefully](/crates/oxide-library/tests/ai_stub/extract_pinout_handles_corrupt_bytes_gracefully.md) |
| called_by | [extract_pinout_is_pure_no_panic_on_empty_input](/crates/oxide-library/tests/ai_stub/extract_pinout_is_pure_no_panic_on_empty_input.md) |
| called_by | [garbage_fixture_returns_no_pins_with_low_confidence](/crates/oxide-library/tests/ai_stub/garbage_fixture_returns_no_pins_with_low_confidence.md) |
| called_by | [lm317_fixture_returns_three_pins_with_high_confidence](/crates/oxide-library/tests/ai_stub/lm317_fixture_returns_three_pins_with_high_confidence.md) |

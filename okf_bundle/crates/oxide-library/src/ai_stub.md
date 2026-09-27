---
okf_version: "0.2"
type: Module
title: ai_stub
description: Heuristic pinout extractor — datasheet PDF → guessed pin list.
resource: crates/oxide-library/src/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/ai_stub
language: rust
---

# ai_stub

Heuristic pinout extractor — datasheet PDF → guessed pin list.

## Docstring

Heuristic pinout extractor — datasheet PDF → guessed pin list.

Per v0.9-library-plan.md §11.3 the **real** LLM-based symbol synthesis is deferred
to v3.x. This module provides a deliberately dumb, deterministic stub:

1. Run [`pdf_extract::extract_text_from_mem`] over the bytes.
2. Walk the text line-by-line looking for "pinout table" rows of the
shape `<pin#> <name> <type/description>`.
3. Classify each row by keyword (`OUT*`, `IN*`, `GND/VCC`, …).
4. Return a [`PinoutGuess`] with a confidence score in `[0, 1]`.

Hard guarantees:
* **No network calls.**
* **No LLM API usage.**
* **No `reqwest` / `oauth2`.**

Caller policy: the Component Editor shows a "Low confidence — review
manually" banner whenever `confidence < 0.5`.

Feature gate: `ai-stub`.

## Relationships

| Type | Target |
|------|--------|
| related | [PinGuess](/crates/oxide-library/src/ai_stub/PinGuess.md) |
| related | [PinoutGuess](/crates/oxide-library/src/ai_stub/PinoutGuess.md) |
| related | [extract_pinout](/crates/oxide-library/src/ai_stub/extract_pinout.md) |
| related | [extract_pinout_from_text](/crates/oxide-library/src/ai_stub/extract_pinout_from_text.md) |
| related | [parse_pin_row](/crates/oxide-library/src/ai_stub/parse_pin_row.md) |
| related | [classify_pin_kind](/crates/oxide-library/src/ai_stub/classify_pin_kind.md) |
| related | [score_confidence](/crates/oxide-library/src/ai_stub/score_confidence.md) |
| related | [pin_row_regex](/crates/oxide-library/src/ai_stub/pin_row_regex.md) |
| related | [parses_lm317_style_table_from_plain_text](/crates/oxide-library/src/ai_stub/parses_lm317_style_table_from_plain_text.md) |
| related | [empty_text_yields_zero_confidence](/crates/oxide-library/src/ai_stub/empty_text_yields_zero_confidence.md) |
| related | [garbage_text_yields_low_confidence](/crates/oxide-library/src/ai_stub/garbage_text_yields_low_confidence.md) |
| related | [classifies_power_input_output_passive](/crates/oxide-library/src/ai_stub/classifies_power_input_output_passive.md) |
| related | [rejects_long_garbage_token_as_pin_name](/crates/oxide-library/src/ai_stub/rejects_long_garbage_token_as_pin_name.md) |
| related | [dedupes_repeated_pin_numbers](/crates/oxide-library/src/ai_stub/dedupes_repeated_pin_numbers.md) |
| related | [pinout_guess_round_trips_via_serde](/crates/oxide-library/src/ai_stub/pinout_guess_round_trips_via_serde.md) |
| related | [regex](/_dependencies/cargo/regex.md) |
| related | [serde](/_dependencies/cargo/serde.md) |

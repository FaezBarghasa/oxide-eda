---
okf_version: "0.2"
type: Function
title: score_confidence
description: "Compute confidence in `[0, 1]` from the parsed pin list and the source"
resource: crates/oxide-library/src/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/ai_stub/score_confidence
language: rust
---

# score_confidence

Compute confidence in `[0, 1]` from the parsed pin list and the source

## Signature

```rust
fn score_confidence(pins: &[PinGuess], text: &str) -> f32
```

## Docstring

Compute confidence in `[0, 1]` from the parsed pin list and the source
text.

Heuristic — reward:
* having ≥ 2 pins (pinouts are never 1-pin)
* the source text containing pinout-y vocabulary
* pin numbers being a contiguous-ish sequence (1, 2, 3 …)

Penalise:
* empty pin list → near zero
* a single pin → cap at 0.3

## Source
Lines 162–207 in `crates/oxide-library/src/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/src/ai_stub.md) |
| called_by | [extract_pinout_from_text](/crates/oxide-library/src/ai_stub/extract_pinout_from_text.md) |

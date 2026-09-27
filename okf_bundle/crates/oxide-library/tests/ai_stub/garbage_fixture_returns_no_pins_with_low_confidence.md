---
okf_version: "0.2"
type: Function
title: garbage_fixture_returns_no_pins_with_low_confidence
description: "[test]"
resource: crates/oxide-library/tests/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/ai_stub/garbage_fixture_returns_no_pins_with_low_confidence
language: rust
---

# garbage_fixture_returns_no_pins_with_low_confidence

[test]

## Signature

```rust
fn garbage_fixture_returns_no_pins_with_low_confidence()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 152–174 in `crates/oxide-library/tests/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/tests/ai_stub.md) |
| calls | [ensure_fixtures](/crates/oxide-library/tests/ai_stub/ensure_fixtures.md) |
| calls | [fixtures_dir](/crates/oxide-library/tests/ai_stub/fixtures_dir.md) |
| calls | [extract_pinout](/crates/oxide-library/src/ai_stub/extract_pinout.md) |

---
okf_version: "0.2"
type: Function
title: ensure_fixtures
resource: crates/oxide-library/tests/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/ai_stub/ensure_fixtures
language: rust
---

# ensure_fixtures

## Signature

```rust
fn ensure_fixtures()
```

## Source
Lines 22–37 in `crates/oxide-library/tests/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/tests/ai_stub.md) |
| calls | [fixtures_dir](/crates/oxide-library/tests/ai_stub/fixtures_dir.md) |
| calls | [generate_lm317_pdf](/crates/oxide-library/tests/ai_stub/generate_lm317_pdf.md) |
| calls | [generate_garbage_pdf](/crates/oxide-library/tests/ai_stub/generate_garbage_pdf.md) |
| called_by | [garbage_fixture_returns_no_pins_with_low_confidence](/crates/oxide-library/tests/ai_stub/garbage_fixture_returns_no_pins_with_low_confidence.md) |
| called_by | [lm317_fixture_returns_three_pins_with_high_confidence](/crates/oxide-library/tests/ai_stub/lm317_fixture_returns_three_pins_with_high_confidence.md) |

---
okf_version: "0.2"
type: Module
title: tests
description: Unit tests for the Symbol-tab.
resource: crates/oxide-app/src/library/editor/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/tests
language: rust
---

# tests

Unit tests for the Symbol-tab.

## Docstring

Unit tests for the Symbol-tab.

Coverage of the typed `Symbol` primitive helpers (`add_pin`,
`move_selected`, `delete_selected`, `apply_ai_pinout`) lives
alongside the helpers in `state.rs`. This file keeps a single
AI-stub low-confidence assertion to ensure the bridge between
the AI guess and the typed primitive still flags suspicious
heuristics.

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub_default_preview_is_low_confidence](/crates/oxide-app/src/library/editor/symbol/tests/ai_stub_default_preview_is_low_confidence.md) |

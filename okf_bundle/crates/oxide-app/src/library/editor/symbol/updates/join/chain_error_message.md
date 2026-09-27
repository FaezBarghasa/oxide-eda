---
okf_version: "0.2"
type: Function
title: chain_error_message
description: Human-readable status-line text for a failed join attempt.
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/chain_error_message
language: rust
---

# chain_error_message

Human-readable status-line text for a failed join attempt.

## Signature

```rust
fn chain_error_message(err: ChainError) -> String
```

## Docstring

Human-readable status-line text for a failed join attempt.

## Source
Lines 169–189 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| called_by | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |

---
okf_version: "0.2"
type: Function
title: scan_token
description: "Try to scan an identifier followed by `}` starting at `start`. Returns"
resource: crates/oxide-output/src/substitution.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/substitution/scan_token
language: rust
---

# scan_token

Try to scan an identifier followed by `}` starting at `start`. Returns

## Signature

```rust
fn scan_token(bytes: &[u8], start: usize) -> Option<(&str, usize)>
```

## Docstring

Try to scan an identifier followed by `}` starting at `start`. Returns
`(identifier, close_brace_index)` on success, `None` if what follows
isn't a valid identifier terminated by `}`.

## Source
Lines 107–126 in `crates/oxide-output/src/substitution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [substitution](/crates/oxide-output/src/substitution.md) |
| calls | [is_ident_start](/crates/oxide-output/src/substitution/is_ident_start.md) |
| calls | [is_ident_continue](/crates/oxide-output/src/substitution/is_ident_continue.md) |
| called_by | [resolve](/crates/oxide-output/src/substitution/resolve.md) |

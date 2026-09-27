---
okf_version: "0.2"
type: Function
title: resolve_ring_with_auto_close
description: "Chain `segments` into a closed ring, auto-closing exactly once by"
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/resolve_ring_with_auto_close
language: rust
---

# resolve_ring_with_auto_close

Chain `segments` into a closed ring, auto-closing exactly once by

## Signature

```rust
fn resolve_ring_with_auto_close(
    segments: &[ChainSegment],
) -> Result<(Vec<[f64; 2]>, Option<f64>), ChainError>
```

## Docstring

Chain `segments` into a closed ring, auto-closing exactly once by
synthesizing the missing edge between an [`ChainError::OpenChain`]'s
two loose ends if the first attempt doesn't close. `Some(gap_mm)`
in the success tuple means auto-close actually fired (so the
caller can surface it as an informational status message);
`None` means the selection was already a closed chain.

## Source
Lines 84–99 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [chain_into_closed_contour](/crates/oxide-library/src/primitive/symbol/chain/chain_into_closed_contour.md) |
| called_by | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |

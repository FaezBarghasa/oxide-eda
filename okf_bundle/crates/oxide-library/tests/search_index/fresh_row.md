---
okf_version: "0.2"
type: Function
title: fresh_row
description: ── fixture builders ───────────────────────────────────────────────────
resource: crates/oxide-library/tests/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/search_index/fresh_row
language: rust
---

# fresh_row

── fixture builders ───────────────────────────────────────────────────

## Signature

```rust
fn fresh_row(
    internal_pn: &str,
    mpn: &str,
    manufacturer: &str,
    _description: &str,
    class: &str,
    parameters: ParamMap,
) -> ComponentRow
```

## Docstring

── fixture builders ───────────────────────────────────────────────────

## Source
Lines 42–75 in `crates/oxide-library/tests/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/tests/search_index.md) |
| called_by | [add_or_update_replaces_existing_doc](/crates/oxide-library/tests/search_index/add_or_update_replaces_existing_doc.md) |
| called_by | [fixture_corpus](/crates/oxide-library/tests/search_index/fixture_corpus.md) |

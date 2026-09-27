---
okf_version: "0.2"
type: Function
title: literal_phrase_query
description: "Re-parse `text` as a quoted phrase over the same default fields, so"
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/literal_phrase_query
language: rust
---

# literal_phrase_query

Re-parse `text` as a quoted phrase over the same default fields, so

## Signature

```rust
fn literal_phrase_query(
    parser: &QueryParser,
    text: &str,
) -> Result<Box<dyn Query>, TantivyIndexError>
```

## Docstring

Re-parse `text` as a quoted phrase over the same default fields, so
query-syntax characters inside a part number are matched as text
rather than interpreted as operators.

Quoting is the escape: `"` and `\` are the only characters Tantivy
still reads specially inside a phrase, and blanking them leaves the
surrounding tokens intact. When even the quoted form will not parse
— text that tokenises to nothing, for instance — this errors rather
than widening the query, because `query()` turns an error into an
honest empty result and a widened query into a false one.

## Source
Lines 736–747 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
| called_by | [build_query](/crates/oxide-library/src/search_index/build_query.md) |

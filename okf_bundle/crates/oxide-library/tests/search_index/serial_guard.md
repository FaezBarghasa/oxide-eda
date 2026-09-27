---
okf_version: "0.2"
type: Function
title: serial_guard
resource: crates/oxide-library/tests/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/search_index/serial_guard
language: rust
---

# serial_guard

## Signature

```rust
fn serial_guard() -> std::sync::MutexGuard<'static, ()>
```

## Source
Lines 34–38 in `crates/oxide-library/tests/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/tests/search_index.md) |
| called_by | [add_or_update_replaces_existing_doc](/crates/oxide-library/tests/search_index/add_or_update_replaces_existing_doc.md) |
| called_by | [category_only_query_filters_corpus](/crates/oxide-library/tests/search_index/category_only_query_filters_corpus.md) |
| called_by | [index_persists_across_drop_and_reopen](/crates/oxide-library/tests/search_index/index_persists_across_drop_and_reopen.md) |
| called_by | [numeric_facet_lt_returns_only_sub_threshold_parts](/crates/oxide-library/tests/search_index/numeric_facet_lt_returns_only_sub_threshold_parts.md) |
| called_by | [text_query_pinpoints_the_single_matching_part](/crates/oxide-library/tests/search_index/text_query_pinpoints_the_single_matching_part.md) |
| called_by | [unparseable_query_text_matches_literally_instead_of_returning_the_library](/crates/oxide-library/tests/search_index/unparseable_query_text_matches_literally_instead_of_returning_the_library.md) |
| called_by | [unparseable_query_text_with_no_match_returns_nothing](/crates/oxide-library/tests/search_index/unparseable_query_text_with_no_match_returns_nothing.md) |

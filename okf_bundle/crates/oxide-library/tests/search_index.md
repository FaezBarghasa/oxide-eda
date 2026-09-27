---
okf_version: "0.2"
type: Module
title: search_index
description: "Acceptance tests for `TantivySearchIndex`."
resource: crates/oxide-library/tests/search_index.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/search_index
language: rust
---

# search_index

Acceptance tests for `TantivySearchIndex`.

## Docstring

Acceptance tests for `TantivySearchIndex`.

The full Tantivy rewrite for the DBLib model is deferred (see
`docs/internal/docs/v0.9-library-plan.md`); the index needs only to
compile and accept [`ComponentRow`] payloads for now. The
corpus-level tests below build rows directly and verify
text/numeric query paths against the row schema.

Run with: `cargo test -p oxide-library --features search-tantivy --test search_index`.

## Relationships

| Type | Target |
|------|--------|
| related | [serial_guard](/crates/oxide-library/tests/search_index/serial_guard.md) |
| related | [fresh_row](/crates/oxide-library/tests/search_index/fresh_row.md) |
| related | [fixture_corpus](/crates/oxide-library/tests/search_index/fixture_corpus.md) |
| related | [text_query_pinpoints_the_single_matching_part](/crates/oxide-library/tests/search_index/text_query_pinpoints_the_single_matching_part.md) |
| related | [numeric_facet_lt_returns_only_sub_threshold_parts](/crates/oxide-library/tests/search_index/numeric_facet_lt_returns_only_sub_threshold_parts.md) |
| related | [index_persists_across_drop_and_reopen](/crates/oxide-library/tests/search_index/index_persists_across_drop_and_reopen.md) |
| related | [add_or_update_replaces_existing_doc](/crates/oxide-library/tests/search_index/add_or_update_replaces_existing_doc.md) |
| related | [category_only_query_filters_corpus](/crates/oxide-library/tests/search_index/category_only_query_filters_corpus.md) |
| related | [unparseable_query_text_matches_literally_instead_of_returning_the_library](/crates/oxide-library/tests/search_index/unparseable_query_text_matches_literally_instead_of_returning_the_library.md) |
| related | [unparseable_query_text_with_no_match_returns_nothing](/crates/oxide-library/tests/search_index/unparseable_query_text_with_no_match_returns_nothing.md) |
| related | [_force_use_btreemap](/crates/oxide-library/tests/search_index/force_use_btreemap.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |

# search_index

## Functions

- [_force_use_btreemap](force_use_btreemap.md) — Defensive: keep an unused `BTreeMap` import in scope so cargo doesn't warn
- [add_or_update_replaces_existing_doc](add_or_update_replaces_existing_doc.md) — [test]
- [category_only_query_filters_corpus](category_only_query_filters_corpus.md) — [test]
- [fixture_corpus](fixture_corpus.md) — Build 100 fixture rows: caps, resistors, ICs, etc.
- [fresh_row](fresh_row.md) — ── fixture builders ───────────────────────────────────────────────────
- [index_persists_across_drop_and_reopen](index_persists_across_drop_and_reopen.md) — [test]
- [numeric_facet_lt_returns_only_sub_threshold_parts](numeric_facet_lt_returns_only_sub_threshold_parts.md) — [test]
- [serial_guard](serial_guard.md)
- [text_query_pinpoints_the_single_matching_part](text_query_pinpoints_the_single_matching_part.md) — [test]
- [unparseable_query_text_matches_literally_instead_of_returning_the_library](unparseable_query_text_matches_literally_instead_of_returning_the_library.md) — Tantivy reads `( ) + - : " [ ] ^ *` as operators, and real part
- [unparseable_query_text_with_no_match_returns_nothing](unparseable_query_text_with_no_match_returns_nothing.md) — A query whose text survives quoting but matches nothing must come

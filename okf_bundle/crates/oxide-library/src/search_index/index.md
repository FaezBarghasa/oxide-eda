# search_index

## Classs

- [SchemaFields](SchemaFields.md) — [derive(Clone)]
- [TantivyIndexError](TantivyIndexError.md) — [derive(Debug, thiserror::Error)]
- [TantivySearchIndex](TantivySearchIndex.md) — Tantivy-backed [`SearchIndex`].
- [UnreadableHits](UnreadableHits.md) — Tally of search hits whose stored document could not be read back

## Functions

- [a_clean_batch_keeps_every_document_and_tallies_nothing](a_clean_batch_keeps_every_document_and_tallies_nothing.md) — [test]
- [add_or_update](add_or_update.md) — Add or replace the doc for a single row.
- [add_or_update](add_or_update_1.md) — Add or replace the doc for a single row.
- [build](build.md)
- [build](build_1.md)
- [build_query](build_query.md)
- [build_query](build_query_1.md)
- [commit](commit.md) — Flush pending writes; required before queries see new docs.
- [commit](commit_1.md) — Flush pending writes; required before queries see new docs.
- [doc_to_summary](doc_to_summary.md)
- [doc_to_summary](doc_to_summary_1.md)
- [facet_to_query](facet_to_query.md)
- [facet_to_query](facet_to_query_1.md)
- [field_name_owned](field_name_owned.md)
- [field_name_owned](field_name_owned_1.md)
- [json_param_query](json_param_query.md)
- [json_param_query](json_param_query_1.md)
- [lifecycle_token](lifecycle_token.md)
- [literal_phrase_query](literal_phrase_query.md) — Re-parse `text` as a quoted phrase over the same default fields, so
- [numeric_param_field_name](numeric_param_field_name.md) — Canonical Tantivy field name for a numeric parameter key.
- [numeric_param_query](numeric_param_query.md)
- [numeric_param_query](numeric_param_query_1.md)
- [open](open.md) — Open or create a Tantivy index rooted at `path`.
- [open](open_1.md) — Open or create a Tantivy index rooted at `path`.
- [parse_f64](parse_f64.md)
- [parse_lifecycle_token](parse_lifecycle_token.md)
- [query](query.md)
- [query](query_1.md)
- [read_error](read_error.md)
- [read_text](read_text.md)
- [read_u64](read_u64.md)
- [split_readable_docs](split_readable_docs.md) — Split a batch of stored-document reads into the documents that came
- [top_level_facet_field](top_level_facet_field.md)
- [top_level_facet_field](top_level_facet_field_1.md)
- [top_level_facet_query](top_level_facet_query.md)
- [top_level_facet_query](top_level_facet_query_1.md)
- [unreadable_documents_are_counted_and_the_first_error_survives](unreadable_documents_are_counted_and_the_first_error_survives.md) — The dropped rows are the whole finding: without a tally the
- [wipe_and_recreate](wipe_and_recreate.md) — M10: nuke the on-disk index at `path` and re-open with the current
- [wipe_and_recreate](wipe_and_recreate_1.md) — M10: nuke the on-disk index at `path` and re-open with the current
- [writer](writer.md)
- [writer](writer_1.md)

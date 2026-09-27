---
okf_version: "0.2"
type: Module
title: search_index
description: "Tantivy-backed implementation of [`SearchIndex`]."
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index
language: rust
---

# search_index

Tantivy-backed implementation of [`SearchIndex`].

## Docstring

Tantivy-backed implementation of [`SearchIndex`].

Schema:
- `uuid`         STRING | STORED  (primary key — used for delete-then-add updates)
- `internal_pn`  TEXT   | STORED
- `mpn`          TEXT   | STORED
- `manufacturer` TEXT   | STORED
- `description`  TEXT   | STORED
- `category`     STRING | STORED  (exact-match facet; surfaced from the
shared parameter `category`)
- `head_major`, `head_minor`  u64 INDEXED | STORED | FAST
- `state`        STRING | STORED  (`LifecycleState` JSON name)
- `parameters`   JSON   | STORED | TEXT  (text parameters; supports
`parameters.dielectric:X7R`-style equality/contains queries via the
`QueryParser`)
- **`param_<key>`** f64  INDEXED | STORED | FAST  (one per well-known
numeric parameter key — see [`NUMERIC_PARAM_KEYS`]; supports `RangeQuery`)

Tantivy 0.22 does **not** support range queries on JSON-field subpaths via
the QueryParser, so each numeric parameter key gets its own typed f64
column in the schema. Adding a new numeric param key requires extending
[`NUMERIC_PARAM_KEYS`] and rebuilding the index from scratch (the schema
check at [`TantivySearchIndex::open`] catches mismatches).

Persisted to whatever directory the caller passes — typically
`<snxlib>/index/search.tantivy/`.

## Relationships

| Type | Target |
|------|--------|
| related | [TantivyIndexError](/crates/oxide-library/src/search_index/TantivyIndexError.md) |
| related | [SchemaFields](/crates/oxide-library/src/search_index/SchemaFields.md) |
| related | [build](/crates/oxide-library/src/search_index/build.md) |
| related | [build](/crates/oxide-library/src/search_index/build.md) |
| related | [numeric_param_field_name](/crates/oxide-library/src/search_index/numeric_param_field_name.md) |
| related | [TantivySearchIndex](/crates/oxide-library/src/search_index/TantivySearchIndex.md) |
| related | [open](/crates/oxide-library/src/search_index/open.md) |
| related | [wipe_and_recreate](/crates/oxide-library/src/search_index/wipe_and_recreate.md) |
| related | [writer](/crates/oxide-library/src/search_index/writer.md) |
| related | [add_or_update](/crates/oxide-library/src/search_index/add_or_update.md) |
| related | [commit](/crates/oxide-library/src/search_index/commit.md) |
| related | [build_query](/crates/oxide-library/src/search_index/build_query.md) |
| related | [facet_to_query](/crates/oxide-library/src/search_index/facet_to_query.md) |
| related | [top_level_facet_field](/crates/oxide-library/src/search_index/top_level_facet_field.md) |
| related | [top_level_facet_query](/crates/oxide-library/src/search_index/top_level_facet_query.md) |
| related | [numeric_param_query](/crates/oxide-library/src/search_index/numeric_param_query.md) |
| related | [json_param_query](/crates/oxide-library/src/search_index/json_param_query.md) |
| related | [field_name_owned](/crates/oxide-library/src/search_index/field_name_owned.md) |
| related | [doc_to_summary](/crates/oxide-library/src/search_index/doc_to_summary.md) |
| related | [open](/crates/oxide-library/src/search_index/open.md) |
| related | [wipe_and_recreate](/crates/oxide-library/src/search_index/wipe_and_recreate.md) |
| related | [writer](/crates/oxide-library/src/search_index/writer.md) |
| related | [add_or_update](/crates/oxide-library/src/search_index/add_or_update.md) |
| related | [commit](/crates/oxide-library/src/search_index/commit.md) |
| related | [build_query](/crates/oxide-library/src/search_index/build_query.md) |
| related | [facet_to_query](/crates/oxide-library/src/search_index/facet_to_query.md) |
| related | [top_level_facet_field](/crates/oxide-library/src/search_index/top_level_facet_field.md) |
| related | [top_level_facet_query](/crates/oxide-library/src/search_index/top_level_facet_query.md) |
| related | [numeric_param_query](/crates/oxide-library/src/search_index/numeric_param_query.md) |
| related | [json_param_query](/crates/oxide-library/src/search_index/json_param_query.md) |
| related | [field_name_owned](/crates/oxide-library/src/search_index/field_name_owned.md) |
| related | [doc_to_summary](/crates/oxide-library/src/search_index/doc_to_summary.md) |
| related | [query](/crates/oxide-library/src/search_index/query.md) |
| related | [query](/crates/oxide-library/src/search_index/query.md) |
| related | [UnreadableHits](/crates/oxide-library/src/search_index/UnreadableHits.md) |
| related | [split_readable_docs](/crates/oxide-library/src/search_index/split_readable_docs.md) |
| related | [literal_phrase_query](/crates/oxide-library/src/search_index/literal_phrase_query.md) |
| related | [parse_f64](/crates/oxide-library/src/search_index/parse_f64.md) |
| related | [lifecycle_token](/crates/oxide-library/src/search_index/lifecycle_token.md) |
| related | [parse_lifecycle_token](/crates/oxide-library/src/search_index/parse_lifecycle_token.md) |
| related | [read_text](/crates/oxide-library/src/search_index/read_text.md) |
| related | [read_u64](/crates/oxide-library/src/search_index/read_u64.md) |
| related | [read_error](/crates/oxide-library/src/search_index/read_error.md) |
| related | [a_clean_batch_keeps_every_document_and_tallies_nothing](/crates/oxide-library/src/search_index/a_clean_batch_keeps_every_document_and_tallies_nothing.md) |
| related | [unreadable_documents_are_counted_and_the_first_error_survives](/crates/oxide-library/src/search_index/unreadable_documents_are_counted_and_the_first_error_survives.md) |
| related | [tantivy](/_dependencies/cargo/tantivy.md) |

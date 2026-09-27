# src

## Subdirectories

- [adapter](adapter/index.md)
- [adapters](adapters/index.md)
- [ai_stub](ai_stub/index.md)
- [cascade](cascade/index.md)
- [component](component/index.md)
- [dependency](dependency/index.md)
- [diff](diff/index.md)
- [distributor](distributor/index.md)
- [distributors](distributors/index.md)
- [harvester](harvester/index.md)
- [hash](hash/index.md)
- [identity](identity/index.md)
- [lib](lib/index.md)
- [library_file](library_file/index.md)
- [lifecycle](lifecycle/index.md)
- [manifest](manifest/index.md)
- [manufacturer](manufacturer/index.md)
- [param](param/index.md)
- [primitive](primitive/index.md)
- [qa](qa/index.md)
- [scraper](scraper/index.md)
- [search](search/index.md)
- [search_index](search_index/index.md)
- [symbol](symbol/index.md)
- [tables](tables/index.md)
- [templates](templates/index.md)
- [where_used](where_used/index.md)

## Modules

- [adapter](adapter.md) — `LibraryAdapter` — the trait every storage flavour implements.
- [ai_stub](ai_stub.md) — Heuristic pinout extractor — datasheet PDF → guessed pin list.
- [cascade](cascade.md) — Primitive-save cascade engine — Stage 15 of `v0.9-snxlib-as-file-plan.md`.
- [component](component.md) — `ComponentRow` — one row of a component table (Altium DBLib model).
- [diff](diff.md) — Pure-data diff between two rows of the same component table.
- [distributor](distributor.md) — `DistributorAdapter` — vendor metadata + pricing lookup. v0.9-library-plan.md §14a.4.
- [hash](hash.md) — Deterministic content hashing for component rows.
- [identity](identity.md)
- [lib](lib.md) — Oxide component library subsystem (v0.9-refactor-2 — DBLib model).
- [lifecycle](lifecycle.md)
- [manifest](manifest.md) — `library.toml` schema for `*.snxlib/` directories. Mirrors v0.9-library-plan.md §13
- [manufacturer](manufacturer.md) — Manufacturer-part + supply-chain types.
- [param](param.md) — Generic parameter map shared by primitives, components, and templates.
- [scraper](scraper.md) — Component Web Scraper, Datasheet Downloader, and Project Library Ingestion.
- [search](search.md) — Faceted parametric search. Tantivy-backed implementation lives in
- [search_index](search_index.md) — Tantivy-backed implementation of [`SearchIndex`].
- [tables](tables.md) — TSV reader/writer for component tables (Altium DBLib model).
- [templates](templates.md) — Parameter templates — class-typed schemas that constrain a component's
- [where_used](where_used.md) — Where-used reverse index — keyed by `RowId` for the DBLib model.

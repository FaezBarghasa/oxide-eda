---
okf_version: "0.2"
type: Module
title: scraper
description: "Component Web Scraper, Datasheet Downloader, and Project Library Ingestion."
resource: crates/oxide-library/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:16:27Z"
concept_id: crates/oxide-library/src/scraper
language: rust
---

# scraper

Component Web Scraper, Datasheet Downloader, and Project Library Ingestion.

## Docstring

Component Web Scraper, Datasheet Downloader, and Project Library Ingestion.

Searches online distributor/component endpoints, parses specifications, downloads and hash-pins
datasheets, synthesizes schematic Symbols and IPC-compliant Footprints, and ingests them into
the local project library (`.snxlib`).

## Relationships

| Type | Target |
|------|--------|
| related | [ScrapedComponent](/crates/oxide-library/src/scraper/ScrapedComponent.md) |
| related | [PackageType](/crates/oxide-library/src/scraper/PackageType.md) |
| related | [from_name](/crates/oxide-library/src/scraper/from_name.md) |
| related | [from_name](/crates/oxide-library/src/scraper/from_name.md) |
| related | [extract_trailing_digits](/crates/oxide-library/src/scraper/extract_trailing_digits.md) |
| related | [synthesize_footprint](/crates/oxide-library/src/scraper/synthesize_footprint.md) |
| related | [synthesize_symbol](/crates/oxide-library/src/scraper/synthesize_symbol.md) |
| related | [ComponentScraper](/crates/oxide-library/src/scraper/ComponentScraper.md) |
| related | [default](/crates/oxide-library/src/scraper/default.md) |
| related | [default](/crates/oxide-library/src/scraper/default.md) |
| related | [new](/crates/oxide-library/src/scraper/new.md) |
| related | [search](/crates/oxide-library/src/scraper/search.md) |
| related | [download_datasheet](/crates/oxide-library/src/scraper/download_datasheet.md) |
| related | [persist_datasheet](/crates/oxide-library/src/scraper/persist_datasheet.md) |
| related | [import_to_library](/crates/oxide-library/src/scraper/import_to_library.md) |
| related | [new](/crates/oxide-library/src/scraper/new.md) |
| related | [search](/crates/oxide-library/src/scraper/search.md) |
| related | [download_datasheet](/crates/oxide-library/src/scraper/download_datasheet.md) |
| related | [persist_datasheet](/crates/oxide-library/src/scraper/persist_datasheet.md) |
| related | [import_to_library](/crates/oxide-library/src/scraper/import_to_library.md) |
| related | [test_package_detection](/crates/oxide-library/src/scraper/test_package_detection.md) |
| related | [test_synthesize_footprint_chip](/crates/oxide-library/src/scraper/test_synthesize_footprint_chip.md) |
| related | [test_synthesize_footprint_soic](/crates/oxide-library/src/scraper/test_synthesize_footprint_soic.md) |
| related | [test_synthesize_symbol](/crates/oxide-library/src/scraper/test_synthesize_symbol.md) |
| related | [test_scraper_search_fallback](/crates/oxide-library/src/scraper/test_scraper_search_fallback.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [sha2](/_dependencies/cargo/sha2.md) |

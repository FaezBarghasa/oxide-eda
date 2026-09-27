---
okf_version: "0.2"
type: Module
title: lcsc
description: "LCSC distributor adapter — anonymous, polite-throttled (1 req/s)."
resource: crates/oxide-library/src/distributors/lcsc.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/lcsc
language: rust
---

# lcsc

LCSC distributor adapter — anonymous, polite-throttled (1 req/s).

## Docstring

LCSC distributor adapter — anonymous, polite-throttled (1 req/s).

- No auth (anonymous public catalogue endpoint).
- **1 req/s** polite throttle. Implemented as a `Mutex<Option<Instant>>`
that delays the next request until at least 1 s after the last one
completed. (Spec mentions `tokio::time::interval`; that fits async,
but `DistributorAdapter` is a sync trait — `std::thread::sleep` is the
sync equivalent and avoids spinning up a runtime per adapter.)
- Disk cache via `DistributorCache` with 24h TTL (per `DEFAULT_TTL`).
- Live API tests are `#[ignore]`d; offline tests use `wiremock` to
verify URL shape, headers, and parsing.

`lookup_by_url` recognises `https://www.lcsc.com/product-detail/<slug>.html`
shapes and extracts the slug; the slug is sent verbatim to LCSC's search
endpoint to obtain the canonical part record.

## Relationships

| Type | Target |
|------|--------|
| related | [LcscAdapter](/crates/oxide-library/src/distributors/lcsc/LcscAdapter.md) |
| related | [new](/crates/oxide-library/src/distributors/lcsc/new.md) |
| related | [with_base_url](/crates/oxide-library/src/distributors/lcsc/with_base_url.md) |
| related | [polite_wait](/crates/oxide-library/src/distributors/lcsc/polite_wait.md) |
| related | [last_call_age](/crates/oxide-library/src/distributors/lcsc/last_call_age.md) |
| related | [http_get_json](/crates/oxide-library/src/distributors/lcsc/http_get_json.md) |
| related | [search_by_keyword](/crates/oxide-library/src/distributors/lcsc/search_by_keyword.md) |
| related | [new](/crates/oxide-library/src/distributors/lcsc/new.md) |
| related | [with_base_url](/crates/oxide-library/src/distributors/lcsc/with_base_url.md) |
| related | [polite_wait](/crates/oxide-library/src/distributors/lcsc/polite_wait.md) |
| related | [last_call_age](/crates/oxide-library/src/distributors/lcsc/last_call_age.md) |
| related | [http_get_json](/crates/oxide-library/src/distributors/lcsc/http_get_json.md) |
| related | [search_by_keyword](/crates/oxide-library/src/distributors/lcsc/search_by_keyword.md) |
| related | [name](/crates/oxide-library/src/distributors/lcsc/name.md) |
| related | [source](/crates/oxide-library/src/distributors/lcsc/source.md) |
| related | [lookup_by_url](/crates/oxide-library/src/distributors/lcsc/lookup_by_url.md) |
| related | [lookup_by_mpn](/crates/oxide-library/src/distributors/lcsc/lookup_by_mpn.md) |
| related | [refresh_pricing](/crates/oxide-library/src/distributors/lcsc/refresh_pricing.md) |
| related | [name](/crates/oxide-library/src/distributors/lcsc/name.md) |
| related | [source](/crates/oxide-library/src/distributors/lcsc/source.md) |
| related | [lookup_by_url](/crates/oxide-library/src/distributors/lcsc/lookup_by_url.md) |
| related | [lookup_by_mpn](/crates/oxide-library/src/distributors/lcsc/lookup_by_mpn.md) |
| related | [refresh_pricing](/crates/oxide-library/src/distributors/lcsc/refresh_pricing.md) |
| related | [urlencoding_minimal](/crates/oxide-library/src/distributors/lcsc/urlencoding_minimal.md) |
| related | [LcscSearchResponse](/crates/oxide-library/src/distributors/lcsc/LcscSearchResponse.md) |
| related | [LcscResult](/crates/oxide-library/src/distributors/lcsc/LcscResult.md) |
| related | [LcscProduct](/crates/oxide-library/src/distributors/lcsc/LcscProduct.md) |
| related | [into_parts](/crates/oxide-library/src/distributors/lcsc/into_parts.md) |
| related | [into_parts](/crates/oxide-library/src/distributors/lcsc/into_parts.md) |
| related | [polite_wait_blocks_until_one_second_passed](/crates/oxide-library/src/distributors/lcsc/polite_wait_blocks_until_one_second_passed.md) |
| related | [lookup_by_url_extracts_lcsc_token](/crates/oxide-library/src/distributors/lcsc/lookup_by_url_extracts_lcsc_token.md) |
| related | [urlencoding_minimal_matches_standard_chars](/crates/oxide-library/src/distributors/lcsc/urlencoding_minimal_matches_standard_chars.md) |
| related | [name_and_source_are_stable](/crates/oxide-library/src/distributors/lcsc/name_and_source_are_stable.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |

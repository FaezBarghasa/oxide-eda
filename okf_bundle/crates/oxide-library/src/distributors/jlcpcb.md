---
okf_version: "0.2"
type: Module
title: jlcpcb
description: "JLCPCB distributor adapter — anonymous, polite-throttled (1 req/s)."
resource: crates/oxide-library/src/distributors/jlcpcb.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/jlcpcb
language: rust
---

# jlcpcb

JLCPCB distributor adapter — anonymous, polite-throttled (1 req/s).

## Docstring

JLCPCB distributor adapter — anonymous, polite-throttled (1 req/s).

Same shape as LCSC — no auth, 1 req/s throttle, disk cache with
24h TTL. JLCPCB's public component search returns an LCSC-style
response (the JLCPCB parts catalogue is a curated subset of LCSC).

## Relationships

| Type | Target |
|------|--------|
| related | [JlcpcbAdapter](/crates/oxide-library/src/distributors/jlcpcb/JlcpcbAdapter.md) |
| related | [new](/crates/oxide-library/src/distributors/jlcpcb/new.md) |
| related | [with_base_url](/crates/oxide-library/src/distributors/jlcpcb/with_base_url.md) |
| related | [polite_wait](/crates/oxide-library/src/distributors/jlcpcb/polite_wait.md) |
| related | [http_post_json](/crates/oxide-library/src/distributors/jlcpcb/http_post_json.md) |
| related | [search_by_keyword](/crates/oxide-library/src/distributors/jlcpcb/search_by_keyword.md) |
| related | [new](/crates/oxide-library/src/distributors/jlcpcb/new.md) |
| related | [with_base_url](/crates/oxide-library/src/distributors/jlcpcb/with_base_url.md) |
| related | [polite_wait](/crates/oxide-library/src/distributors/jlcpcb/polite_wait.md) |
| related | [http_post_json](/crates/oxide-library/src/distributors/jlcpcb/http_post_json.md) |
| related | [search_by_keyword](/crates/oxide-library/src/distributors/jlcpcb/search_by_keyword.md) |
| related | [name](/crates/oxide-library/src/distributors/jlcpcb/name.md) |
| related | [source](/crates/oxide-library/src/distributors/jlcpcb/source.md) |
| related | [lookup_by_url](/crates/oxide-library/src/distributors/jlcpcb/lookup_by_url.md) |
| related | [lookup_by_mpn](/crates/oxide-library/src/distributors/jlcpcb/lookup_by_mpn.md) |
| related | [refresh_pricing](/crates/oxide-library/src/distributors/jlcpcb/refresh_pricing.md) |
| related | [name](/crates/oxide-library/src/distributors/jlcpcb/name.md) |
| related | [source](/crates/oxide-library/src/distributors/jlcpcb/source.md) |
| related | [lookup_by_url](/crates/oxide-library/src/distributors/jlcpcb/lookup_by_url.md) |
| related | [lookup_by_mpn](/crates/oxide-library/src/distributors/jlcpcb/lookup_by_mpn.md) |
| related | [refresh_pricing](/crates/oxide-library/src/distributors/jlcpcb/refresh_pricing.md) |
| related | [JlcpcbResponse](/crates/oxide-library/src/distributors/jlcpcb/JlcpcbResponse.md) |
| related | [JlcpcbData](/crates/oxide-library/src/distributors/jlcpcb/JlcpcbData.md) |
| related | [JlcpcbItem](/crates/oxide-library/src/distributors/jlcpcb/JlcpcbItem.md) |
| related | [into_parts](/crates/oxide-library/src/distributors/jlcpcb/into_parts.md) |
| related | [into_parts](/crates/oxide-library/src/distributors/jlcpcb/into_parts.md) |
| related | [name_and_source_are_stable](/crates/oxide-library/src/distributors/jlcpcb/name_and_source_are_stable.md) |
| related | [lookup_by_url_rejects_non_jlcpcb_host](/crates/oxide-library/src/distributors/jlcpcb/lookup_by_url_rejects_non_jlcpcb_host.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |

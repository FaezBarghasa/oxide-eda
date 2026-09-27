---
okf_version: "0.2"
type: Module
title: mouser
description: Mouser distributor adapter — API-key auth from OS keyring.
resource: crates/oxide-library/src/distributors/mouser.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/mouser
language: rust
---

# mouser

Mouser distributor adapter — API-key auth from OS keyring.

## Docstring

Mouser distributor adapter — API-key auth from OS keyring.

- API key stored in OS keyring under service name
`oxide-distributor-mouser`. Adapter accepts the key directly via
`with_api_key` (test-friendly) or pulls it lazily from `KeyringStore`.
- Mouser's Search API takes JSON POST bodies with an `apiKey` query
string parameter. We use that placement (vs header) for spec parity
with Mouser's documented flow; the spec calls this "header auth" but
Mouser actually accepts both — we use the simpler query form.
- 24h disk cache, polite throttle, same error-mapping as LCSC.

## Relationships

| Type | Target |
|------|--------|
| related | [AuthSource](/crates/oxide-library/src/distributors/mouser/AuthSource.md) |
| related | [MouserAdapter](/crates/oxide-library/src/distributors/mouser/MouserAdapter.md) |
| related | [from_keyring](/crates/oxide-library/src/distributors/mouser/from_keyring.md) |
| related | [with_api_key](/crates/oxide-library/src/distributors/mouser/with_api_key.md) |
| related | [polite_wait](/crates/oxide-library/src/distributors/mouser/polite_wait.md) |
| related | [resolve_api_key](/crates/oxide-library/src/distributors/mouser/resolve_api_key.md) |
| related | [search_by_keyword](/crates/oxide-library/src/distributors/mouser/search_by_keyword.md) |
| related | [from_keyring](/crates/oxide-library/src/distributors/mouser/from_keyring.md) |
| related | [with_api_key](/crates/oxide-library/src/distributors/mouser/with_api_key.md) |
| related | [polite_wait](/crates/oxide-library/src/distributors/mouser/polite_wait.md) |
| related | [resolve_api_key](/crates/oxide-library/src/distributors/mouser/resolve_api_key.md) |
| related | [search_by_keyword](/crates/oxide-library/src/distributors/mouser/search_by_keyword.md) |
| related | [name](/crates/oxide-library/src/distributors/mouser/name.md) |
| related | [source](/crates/oxide-library/src/distributors/mouser/source.md) |
| related | [lookup_by_url](/crates/oxide-library/src/distributors/mouser/lookup_by_url.md) |
| related | [lookup_by_mpn](/crates/oxide-library/src/distributors/mouser/lookup_by_mpn.md) |
| related | [refresh_pricing](/crates/oxide-library/src/distributors/mouser/refresh_pricing.md) |
| related | [name](/crates/oxide-library/src/distributors/mouser/name.md) |
| related | [source](/crates/oxide-library/src/distributors/mouser/source.md) |
| related | [lookup_by_url](/crates/oxide-library/src/distributors/mouser/lookup_by_url.md) |
| related | [lookup_by_mpn](/crates/oxide-library/src/distributors/mouser/lookup_by_mpn.md) |
| related | [refresh_pricing](/crates/oxide-library/src/distributors/mouser/refresh_pricing.md) |
| related | [MouserResponse](/crates/oxide-library/src/distributors/mouser/MouserResponse.md) |
| related | [MouserSearchResults](/crates/oxide-library/src/distributors/mouser/MouserSearchResults.md) |
| related | [MouserPartDto](/crates/oxide-library/src/distributors/mouser/MouserPartDto.md) |
| related | [into_parts](/crates/oxide-library/src/distributors/mouser/into_parts.md) |
| related | [into_parts](/crates/oxide-library/src/distributors/mouser/into_parts.md) |
| related | [name_and_source_are_stable](/crates/oxide-library/src/distributors/mouser/name_and_source_are_stable.md) |
| related | [lookup_by_url_rejects_non_mouser_host](/crates/oxide-library/src/distributors/mouser/lookup_by_url_rejects_non_mouser_host.md) |
| related | [missing_keyring_key_yields_auth_error](/crates/oxide-library/src/distributors/mouser/missing_keyring_key_yields_auth_error.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |

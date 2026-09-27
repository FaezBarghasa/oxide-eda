---
okf_version: "0.2"
type: Module
title: digikey
description: DigiKey distributor adapter — OAuth2 PKCE scaffold.
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey
language: rust
---

# digikey

DigiKey distributor adapter — OAuth2 PKCE scaffold.

## Docstring

DigiKey distributor adapter — OAuth2 PKCE scaffold.

- Uses the `oauth2` crate (v5) for the authorization-code + PKCE flow.
- Refresh token persisted in OS keyring under
`oxide-distributor-digikey` (username slot `"refresh"`).
- No real auth in tests; the refresh-token → access-token exchange is
mocked with `wiremock`. Live API tests are `#[ignore]`d.

Public surface:
- [`DigiKeyAuth`] — orchestrates the OAuth2 flow. The interactive
"open browser, redirect, exchange code" handshake belongs in the UI;
the library just exposes:
* [`DigiKeyAuth::start_authorization()`] — produces the auth URL
plus the PKCE verifier the UI needs to keep until callback.
* [`DigiKeyAuth::exchange_code()`] — UI passes the redirected `code`
here, we persist the refresh token in keyring and return the
access token.
* [`DigiKeyAuth::access_token()`] — refresh-token-grant, called by
the adapter on every request.
- [`DigiKeyAdapter`] — implements `DistributorAdapter` using the
access token in `Authorization: Bearer …`.

## Relationships

| Type | Target |
|------|--------|
| related | [DigiKeyAuthError](/crates/oxide-library/src/distributors/digikey/DigiKeyAuthError.md) |
| related | [from](/crates/oxide-library/src/distributors/digikey/from.md) |
| related | [from](/crates/oxide-library/src/distributors/digikey/from.md) |
| related | [DigiKeyAuth](/crates/oxide-library/src/distributors/digikey/DigiKeyAuth.md) |
| related | [new](/crates/oxide-library/src/distributors/digikey/new.md) |
| related | [with_endpoints](/crates/oxide-library/src/distributors/digikey/with_endpoints.md) |
| related | [with_test_refresh_token](/crates/oxide-library/src/distributors/digikey/with_test_refresh_token.md) |
| related | [start_authorization](/crates/oxide-library/src/distributors/digikey/start_authorization.md) |
| related | [exchange_code](/crates/oxide-library/src/distributors/digikey/exchange_code.md) |
| related | [access_token](/crates/oxide-library/src/distributors/digikey/access_token.md) |
| related | [new](/crates/oxide-library/src/distributors/digikey/new.md) |
| related | [with_endpoints](/crates/oxide-library/src/distributors/digikey/with_endpoints.md) |
| related | [with_test_refresh_token](/crates/oxide-library/src/distributors/digikey/with_test_refresh_token.md) |
| related | [start_authorization](/crates/oxide-library/src/distributors/digikey/start_authorization.md) |
| related | [exchange_code](/crates/oxide-library/src/distributors/digikey/exchange_code.md) |
| related | [access_token](/crates/oxide-library/src/distributors/digikey/access_token.md) |
| related | [build_oauth_client](/crates/oxide-library/src/distributors/digikey/build_oauth_client.md) |
| related | [build_http_client](/crates/oxide-library/src/distributors/digikey/build_http_client.md) |
| related | [DigiKeyAdapter](/crates/oxide-library/src/distributors/digikey/DigiKeyAdapter.md) |
| related | [AuthSource](/crates/oxide-library/src/distributors/digikey/AuthSource.md) |
| related | [new](/crates/oxide-library/src/distributors/digikey/new.md) |
| related | [with_access_token](/crates/oxide-library/src/distributors/digikey/with_access_token.md) |
| related | [with_oauth_and_base](/crates/oxide-library/src/distributors/digikey/with_oauth_and_base.md) |
| related | [polite_wait](/crates/oxide-library/src/distributors/digikey/polite_wait.md) |
| related | [resolve_access_token](/crates/oxide-library/src/distributors/digikey/resolve_access_token.md) |
| related | [search_by_keyword](/crates/oxide-library/src/distributors/digikey/search_by_keyword.md) |
| related | [new](/crates/oxide-library/src/distributors/digikey/new.md) |
| related | [with_access_token](/crates/oxide-library/src/distributors/digikey/with_access_token.md) |
| related | [with_oauth_and_base](/crates/oxide-library/src/distributors/digikey/with_oauth_and_base.md) |
| related | [polite_wait](/crates/oxide-library/src/distributors/digikey/polite_wait.md) |
| related | [resolve_access_token](/crates/oxide-library/src/distributors/digikey/resolve_access_token.md) |
| related | [search_by_keyword](/crates/oxide-library/src/distributors/digikey/search_by_keyword.md) |
| related | [name](/crates/oxide-library/src/distributors/digikey/name.md) |
| related | [source](/crates/oxide-library/src/distributors/digikey/source.md) |
| related | [lookup_by_url](/crates/oxide-library/src/distributors/digikey/lookup_by_url.md) |
| related | [lookup_by_mpn](/crates/oxide-library/src/distributors/digikey/lookup_by_mpn.md) |
| related | [refresh_pricing](/crates/oxide-library/src/distributors/digikey/refresh_pricing.md) |
| related | [name](/crates/oxide-library/src/distributors/digikey/name.md) |
| related | [source](/crates/oxide-library/src/distributors/digikey/source.md) |
| related | [lookup_by_url](/crates/oxide-library/src/distributors/digikey/lookup_by_url.md) |
| related | [lookup_by_mpn](/crates/oxide-library/src/distributors/digikey/lookup_by_mpn.md) |
| related | [refresh_pricing](/crates/oxide-library/src/distributors/digikey/refresh_pricing.md) |
| related | [DigiKeyResponse](/crates/oxide-library/src/distributors/digikey/DigiKeyResponse.md) |
| related | [DigiKeyProductDto](/crates/oxide-library/src/distributors/digikey/DigiKeyProductDto.md) |
| related | [DigiKeyManufacturer](/crates/oxide-library/src/distributors/digikey/DigiKeyManufacturer.md) |
| related | [DigiKeyDescription](/crates/oxide-library/src/distributors/digikey/DigiKeyDescription.md) |
| related | [into_parts](/crates/oxide-library/src/distributors/digikey/into_parts.md) |
| related | [into_parts](/crates/oxide-library/src/distributors/digikey/into_parts.md) |
| related | [name_and_source_are_stable](/crates/oxide-library/src/distributors/digikey/name_and_source_are_stable.md) |
| related | [lookup_by_url_rejects_non_digikey_host](/crates/oxide-library/src/distributors/digikey/lookup_by_url_rejects_non_digikey_host.md) |
| related | [start_authorization_returns_pkce_protected_url](/crates/oxide-library/src/distributors/digikey/start_authorization_returns_pkce_protected_url.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [oauth2](/_dependencies/cargo/oauth2.md) |
| related | [serde](/_dependencies/cargo/serde.md) |

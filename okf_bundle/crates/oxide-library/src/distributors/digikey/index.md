# digikey

## Classs

- [AuthSource](AuthSource.md)
- [DigiKeyAdapter](DigiKeyAdapter.md) — ---------------------------------------------------------------------------
- [DigiKeyAuth](DigiKeyAuth.md) — Scaffolds the OAuth2 PKCE authorization-code flow.
- [DigiKeyAuthError](DigiKeyAuthError.md) — [derive(Debug, thiserror::Error)]
- [DigiKeyDescription](DigiKeyDescription.md) — [derive(Debug, Default, Deserialize)]
- [DigiKeyManufacturer](DigiKeyManufacturer.md) — [derive(Debug, Default, Deserialize)]
- [DigiKeyProductDto](DigiKeyProductDto.md) — [derive(Debug, Deserialize)]
- [DigiKeyResponse](DigiKeyResponse.md) — [derive(Debug, Deserialize)]

## Functions

- [access_token](access_token.md) — Step 3 (every API call): use the keyring-stored refresh token to
- [access_token](access_token_1.md) — Step 3 (every API call): use the keyring-stored refresh token to
- [build_http_client](build_http_client.md)
- [build_oauth_client](build_oauth_client.md)
- [exchange_code](exchange_code.md) — Step 2: exchange the redirected `code` for tokens; persist the
- [exchange_code](exchange_code_1.md) — Step 2: exchange the redirected `code` for tokens; persist the
- [from](from.md)
- [from](from_1.md)
- [into_parts](into_parts.md)
- [into_parts](into_parts_1.md)
- [lookup_by_mpn](lookup_by_mpn.md)
- [lookup_by_mpn](lookup_by_mpn_1.md)
- [lookup_by_url](lookup_by_url.md)
- [lookup_by_url](lookup_by_url_1.md)
- [lookup_by_url_rejects_non_digikey_host](lookup_by_url_rejects_non_digikey_host.md) — [test]
- [name](name.md)
- [name](name_1.md)
- [name_and_source_are_stable](name_and_source_are_stable.md) — [test]
- [new](new.md) — Production constructor: real DigiKey endpoints, refresh token in
- [new](new_1.md) — Production constructor: real DigiKey endpoints, refresh token in
- [new](new_2.md) — Production constructor.
- [new](new_3.md) — Production constructor.
- [polite_wait](polite_wait.md)
- [polite_wait](polite_wait_1.md)
- [refresh_pricing](refresh_pricing.md)
- [refresh_pricing](refresh_pricing_1.md)
- [resolve_access_token](resolve_access_token.md)
- [resolve_access_token](resolve_access_token_1.md)
- [search_by_keyword](search_by_keyword.md)
- [search_by_keyword](search_by_keyword_1.md)
- [source](source.md)
- [source](source_1.md)
- [start_authorization](start_authorization.md) — Step 1 of the flow: produce the authorization URL the UI should open
- [start_authorization](start_authorization_1.md) — Step 1 of the flow: produce the authorization URL the UI should open
- [start_authorization_returns_pkce_protected_url](start_authorization_returns_pkce_protected_url.md) — [test]
- [with_access_token](with_access_token.md) — Test constructor: override the API base + provide a fixed access
- [with_access_token](with_access_token_1.md) — Test constructor: override the API base + provide a fixed access
- [with_endpoints](with_endpoints.md) — Test constructor: override auth + token URLs (e.g. wiremock).
- [with_endpoints](with_endpoints_1.md) — Test constructor: override auth + token URLs (e.g. wiremock).
- [with_oauth_and_base](with_oauth_and_base.md) — Test+production constructor: provide a `DigiKeyAuth` that points at
- [with_oauth_and_base](with_oauth_and_base_1.md) — Test+production constructor: provide a `DigiKeyAuth` that points at
- [with_test_refresh_token](with_test_refresh_token.md) — Test-only setter: provide an in-memory refresh token. When set,
- [with_test_refresh_token](with_test_refresh_token_1.md) — Test-only setter: provide an in-memory refresh token. When set,

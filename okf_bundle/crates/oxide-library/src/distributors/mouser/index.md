# mouser

## Classs

- [AuthSource](AuthSource.md) — How the Mouser adapter retrieves its API key.
- [MouserAdapter](MouserAdapter.md)
- [MouserPartDto](MouserPartDto.md) — [derive(Debug, Deserialize)]
- [MouserResponse](MouserResponse.md) — [derive(Debug, Deserialize)]
- [MouserSearchResults](MouserSearchResults.md) — [derive(Debug, Deserialize)]

## Functions

- [from_keyring](from_keyring.md) — Production constructor: pulls the API key from `oxide-distributor-mouser`
- [from_keyring](from_keyring_1.md) — Production constructor: pulls the API key from `oxide-distributor-mouser`
- [into_parts](into_parts.md)
- [into_parts](into_parts_1.md)
- [lookup_by_mpn](lookup_by_mpn.md)
- [lookup_by_mpn](lookup_by_mpn_1.md)
- [lookup_by_url](lookup_by_url.md)
- [lookup_by_url](lookup_by_url_1.md)
- [lookup_by_url_rejects_non_mouser_host](lookup_by_url_rejects_non_mouser_host.md) — [test]
- [missing_keyring_key_yields_auth_error](missing_keyring_key_yields_auth_error.md) — [test]
- [name](name.md)
- [name](name_1.md)
- [name_and_source_are_stable](name_and_source_are_stable.md) — [test]
- [polite_wait](polite_wait.md)
- [polite_wait](polite_wait_1.md)
- [refresh_pricing](refresh_pricing.md)
- [refresh_pricing](refresh_pricing_1.md)
- [resolve_api_key](resolve_api_key.md)
- [resolve_api_key](resolve_api_key_1.md)
- [search_by_keyword](search_by_keyword.md)
- [search_by_keyword](search_by_keyword_1.md)
- [source](source.md)
- [source](source_1.md)
- [with_api_key](with_api_key.md) — Test constructor: inline API key, override base URL.
- [with_api_key](with_api_key_1.md) — Test constructor: inline API key, override base URL.

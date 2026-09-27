# keyring

## Classs

- [KeyringError](KeyringError.md) — [derive(Debug, thiserror::Error)]
- [KeyringStore](KeyringStore.md) — Wrapper around a single keyring entry, scoped to one distributor provider.

## Functions

- [delete](delete.md) — Delete the entry. Idempotent: deleting an absent entry is `Ok`.
- [delete](delete_1.md) — Delete the entry. Idempotent: deleting an absent entry is `Ok`.
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [for_provider](for_provider.md) — Create a store for `provider` with the given `username` slot.
- [for_provider](for_provider_1.md) — Create a store for `provider` with the given `username` slot.
- [for_provider_builds_expected_service_name](for_provider_builds_expected_service_name.md) — [test]
- [from](from.md)
- [from](from_1.md)
- [get_secret](get_secret.md) — Read the stored secret. Returns `KeyringError::NotFound` if absent.
- [get_secret](get_secret_1.md) — Read the stored secret. Returns `KeyringError::NotFound` if absent.
- [service_name](service_name.md) — Service name as registered with the OS keychain.
- [service_name](service_name_1.md) — Service name as registered with the OS keychain.
- [service_prefix_is_stable](service_prefix_is_stable.md) — [test]
- [set_secret](set_secret.md) — Persist the secret. Overwrites any existing value.
- [set_secret](set_secret_1.md) — Persist the secret. Overwrites any existing value.
- [username](username.md) — Username slot.
- [username](username_1.md) — Username slot.

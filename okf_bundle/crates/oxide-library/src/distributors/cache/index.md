# cache

## Classs

- [CacheError](CacheError.md) — [derive(Debug, thiserror::Error)]
- [DistributorCache](DistributorCache.md) — Filesystem-backed cache of `DistributorPart` JSON, keyed by `(provider, mpn)`.

## Functions

- [default_root](default_root.md) — Resolve `~/.oxide/cache/distributor` (creates it on first use).
- [default_root](default_root_1.md) — Resolve `~/.oxide/cache/distributor` (creates it on first use).
- [entry_path](entry_path.md) — Compute the on-disk path for a `(provider, mpn)` entry.
- [entry_path](entry_path_1.md) — Compute the on-disk path for a `(provider, mpn)` entry.
- [entry_path_rejects_parent_dir_traversal](entry_path_rejects_parent_dir_traversal.md) — M2: an MPN containing `..` must be rejected on every cache op so a
- [entry_path_sanitises_slashes](entry_path_sanitises_slashes.md) — [test]
- [get](get.md) — Read a cached part if it exists and is fresher than `ttl`.
- [get](get_1.md) — Read a cached part if it exists and is fresher than `ttl`.
- [invalidate](invalidate.md) — Delete a cached entry, if present. Idempotent.
- [invalidate](invalidate_1.md) — Delete a cached entry, if present. Idempotent.
- [invalidate_is_idempotent](invalidate_is_idempotent.md) — [test]
- [part](part.md)
- [put](put.md) — Write a part to the cache. Refreshes `captured_at` is the caller's
- [put](put_1.md) — Write a part to the cache. Refreshes `captured_at` is the caller's
- [root](root.md) — Root directory of this cache. Mostly for tests/diagnostics.
- [root](root_1.md) — Root directory of this cache. Mostly for tests/diagnostics.
- [validate_entry_path](validate_entry_path.md) — M2: enforce that `mpn` produces a path strictly inside `self.root`.
- [validate_entry_path](validate_entry_path_1.md) — M2: enforce that `mpn` produces a path strictly inside `self.root`.
- [with_root](with_root.md) — Construct a cache at the given root directory. Creates the root if it
- [with_root](with_root_1.md) — Construct a cache at the given root directory. Creates the root if it

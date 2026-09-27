# locks

## Classs

- [Entry](Entry.md) — [derive(Debug)]
- [Inner](Inner.md)
- [LockError](LockError.md) — [derive(Debug, Clone, PartialEq, Eq)]
- [LockErrorKind](LockErrorKind.md) — [derive(Debug, Clone, PartialEq, Eq)]
- [LockManager](LockManager.md)
- [LockSnapshot](LockSnapshot.md) — [derive(Debug, Clone)]

## Functions

- [different_field_sets_are_independent](different_field_sets_are_independent.md) — [test]
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [new](new.md)
- [new](new_1.md)
- [release](release.md)
- [release](release_1.md)
- [release_by_other_fails](release_by_other_fails.md) — [test]
- [release_then_relock_works](release_then_relock_works.md) — [test]
- [renew](renew.md)
- [renew](renew_1.md)
- [set_idle_ttl](set_idle_ttl.md) — Override TTL — primarily for tests so they don't have to wait minutes.
- [set_idle_ttl](set_idle_ttl_1.md) — Override TTL — primarily for tests so they don't have to wait minutes.
- [snapshot](snapshot.md)
- [snapshot](snapshot_1.md)
- [sweep_expired](sweep_expired.md) — Drop expired entries. Background tasks may call this periodically.
- [sweep_expired](sweep_expired_1.md) — Drop expired entries. Background tasks may call this periodically.
- [try_lock](try_lock.md)
- [try_lock](try_lock_1.md)
- [try_lock_blocks_when_held](try_lock_blocks_when_held.md) — [test]
- [ttl_expiry_allows_takeover](ttl_expiry_allows_takeover.md) — [test]

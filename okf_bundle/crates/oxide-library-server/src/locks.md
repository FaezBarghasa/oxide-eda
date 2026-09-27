---
okf_version: "0.2"
type: Module
title: locks
description: In-memory advisory lock service.
resource: crates/oxide-library-server/src/locks.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library-server/src/locks
language: rust
---

# locks

In-memory advisory lock service.

## Docstring

In-memory advisory lock service.

Locks are keyed on `(Uuid, FieldSet)`. Each lock records the holder
and the wall-clock time it was last touched. A lock is considered free if
its `last_renewed + idle_ttl` has passed — that's the "idle TTL" the spec
calls out (default 10 min, override per-test via `set_idle_ttl`).

The lockable identifier is a row's `RowId`; the manager keeps a
bare `Uuid` so it stays type-agnostic. Callers convert via
`RowId::as_uuid()`.

Persistence is intentionally NOT in the SQL `locks` table by default: the
advisory layer is purely in-memory and cheap to reset on server restart.
The DB table is reserved for a future durable-locks mode (e.g. across
horizontally-scaled replicas).

## Relationships

| Type | Target |
|------|--------|
| related | [LockErrorKind](/crates/oxide-library-server/src/locks/LockErrorKind.md) |
| related | [LockError](/crates/oxide-library-server/src/locks/LockError.md) |
| related | [fmt](/crates/oxide-library-server/src/locks/fmt.md) |
| related | [fmt](/crates/oxide-library-server/src/locks/fmt.md) |
| related | [LockSnapshot](/crates/oxide-library-server/src/locks/LockSnapshot.md) |
| related | [Entry](/crates/oxide-library-server/src/locks/Entry.md) |
| related | [LockManager](/crates/oxide-library-server/src/locks/LockManager.md) |
| related | [Inner](/crates/oxide-library-server/src/locks/Inner.md) |
| related | [new](/crates/oxide-library-server/src/locks/new.md) |
| related | [set_idle_ttl](/crates/oxide-library-server/src/locks/set_idle_ttl.md) |
| related | [try_lock](/crates/oxide-library-server/src/locks/try_lock.md) |
| related | [renew](/crates/oxide-library-server/src/locks/renew.md) |
| related | [release](/crates/oxide-library-server/src/locks/release.md) |
| related | [snapshot](/crates/oxide-library-server/src/locks/snapshot.md) |
| related | [sweep_expired](/crates/oxide-library-server/src/locks/sweep_expired.md) |
| related | [new](/crates/oxide-library-server/src/locks/new.md) |
| related | [set_idle_ttl](/crates/oxide-library-server/src/locks/set_idle_ttl.md) |
| related | [try_lock](/crates/oxide-library-server/src/locks/try_lock.md) |
| related | [renew](/crates/oxide-library-server/src/locks/renew.md) |
| related | [release](/crates/oxide-library-server/src/locks/release.md) |
| related | [snapshot](/crates/oxide-library-server/src/locks/snapshot.md) |
| related | [sweep_expired](/crates/oxide-library-server/src/locks/sweep_expired.md) |
| related | [try_lock_blocks_when_held](/crates/oxide-library-server/src/locks/try_lock_blocks_when_held.md) |
| related | [release_then_relock_works](/crates/oxide-library-server/src/locks/release_then_relock_works.md) |
| related | [ttl_expiry_allows_takeover](/crates/oxide-library-server/src/locks/ttl_expiry_allows_takeover.md) |
| related | [different_field_sets_are_independent](/crates/oxide-library-server/src/locks/different_field_sets_are_independent.md) |
| related | [release_by_other_fails](/crates/oxide-library-server/src/locks/release_by_other_fails.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |

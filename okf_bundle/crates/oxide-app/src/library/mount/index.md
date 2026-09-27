# mount

## Classs

- [MountIntent](MountIntent.md) — Why a mount was requested — decides what the completion handler does
- [MountRequest](MountRequest.md) — What [`LibraryState::request_mount`] decided.
- [PreparedMount](PreparedMount.md) — Everything the UI thread needs to finish a mount, built entirely off
- [PreparedMountCell](PreparedMountCell.md) — One-shot carrier for a [`PreparedMount`] across the `Task::perform`

## Functions

- [adapter_ref](adapter_ref.md) — Widen a concrete adapter to the trait object `reload_tables` takes.
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [mount_prepared](mount_prepared.md) — Finish a mount prepared off-thread. Cheap: a `LibrarySet::mount`
- [mount_prepared](mount_prepared_1.md) — Finish a mount prepared off-thread. Cheap: a `LibrarySet::mount`
- [new](new.md)
- [new](new_1.md)
- [path](path.md) — The `.snxlib` file path this mount was prepared for.
- [path](path_1.md) — The `.snxlib` file path this mount was prepared for.
- [prepare_mount](prepare_mount.md) — Open the adapter and prime every cache — the `spawn_blocking` body.
- [prepare_mount_off_thread](prepare_mount_off_thread.md) — Prepare a mount off the UI thread, wrapped for `Task::perform`.
- [request_mount](request_mount.md) — Record a mount request and report what the caller should do.
- [request_mount](request_mount_1.md) — Record a mount request and report what the caller should do.
- [take](take.md) — Take the payload. `None` on a second read — see the type docs.
- [take](take_1.md) — Take the payload. `None` on a second read — see the type docs.
- [take_mount_intent](take_mount_intent.md) — Take the recorded intent for a finished mount.
- [take_mount_intent](take_mount_intent_1.md) — Take the recorded intent for a finished mount.
- [upgrade](upgrade.md) — Intents only ever escalate. `OpenBrowserTab` absorbs `Silent`;
- [upgrade](upgrade_1.md) — Intents only ever escalate. `OpenBrowserTab` absorbs `Silent`;

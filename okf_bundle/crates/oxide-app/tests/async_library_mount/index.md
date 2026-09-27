# async_library_mount

## Functions

- [a_double_click_during_a_silent_mount_upgrades_the_intent](a_double_click_during_a_silent_mount_upgrades_the_intent.md) — The subtle half of the design. A project auto-mount records `Silent`;
- [a_silent_request_never_downgrades_an_open_browser_tab_intent](a_silent_request_never_downgrades_an_open_browser_tab_intent.md) — The reverse must not happen: a background auto-mount arriving after
- [close_library_cancels_a_mount_still_being_prepared](close_library_cancels_a_mount_still_being_prepared.md) — Closing a library while its preparation is in flight has to cancel it.
- [fixture](fixture.md) — A real `.snxlib` written by the real Oxide writers. Returns the
- [mount_prepared_does_not_duplicate_an_already_mounted_library](mount_prepared_does_not_duplicate_an_already_mounted_library.md) — `mount_prepared` carries the same idempotence guard as
- [prepare_mount_then_mount_prepared_matches_open_library](prepare_mount_then_mount_prepared_matches_open_library.md) — The whole point of the change: preparing off-thread and mounting the
- [request_mount_asks_for_a_spawn_once_then_reports_in_flight](request_mount_asks_for_a_spawn_once_then_reports_in_flight.md) — A cold path with nothing in flight must tell the caller to spawn, and
- [request_mount_reports_already_mounted_and_records_nothing](request_mount_reports_already_mounted_and_records_nothing.md) — An already-mounted library is neither a spawn nor an in-flight wait —
- [take_mount_intent_is_one_shot](take_mount_intent_is_one_shot.md) — `take_mount_intent` consumes. A second completion for the same path

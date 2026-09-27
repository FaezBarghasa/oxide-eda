# bridge

## Functions

- [asking_whether_a_command_resolves_reports_nothing](asking_whether_a_command_resolves_reports_nothing.md) — #619 — a query must not narrate. The palette filters its rows by
- [bridged_command_ids](bridged_command_ids.md)
- [core_to_message](core_to_message.md) — Resolve a command for DISPATCH, reporting when it cannot be resolved.
- [dispatching_an_unmapped_command_still_reports_it](dispatching_an_unmapped_command_still_reports_it.md) — The other half: dispatching an unmapped command still reports, so
- [every_bridged_command_id_resolves_in_the_catalog](every_bridged_command_id_resolves_in_the_catalog.md) — Drift guard: every id `core_to_message` matches must resolve in
- [is_dispatchable](is_dispatchable.md) — Does this command resolve to a message? Silent — no diagnostics.
- [log_unmapped](log_unmapped.md) — Report an id that reached the bridge and found no arm.
- [match_block](match_block.md) — The `core_to_message` match block alone, cut out of `src`.
- [pinned_unmapped_ids_still_exist_in_the_catalog](pinned_unmapped_ids_still_exist_in_the_catalog.md) — The pinned ids must still exist in the catalog, so the list cannot
- [records_naming](records_naming.md) — Records currently in the ring that name this command id.
- [resolvable_command_ids](resolvable_command_ids.md) — Pull every quoted command-id literal out of `src`'s match-arm
- [resolve](resolve.md) — The resolution itself. Pure: no logging, no side effects, so both
- [resolve](resolve_1.md)
- [serial](serial.md) — The diagnostics ring is one process-wide `Mutex<VecDeque<_>>`
- [the_newly_wired_ids_reach_their_named_messages](the_newly_wired_ids_reach_their_named_messages.md) — The ratchet proves the seven ids this slice drained now *resolve*;
- [unmapped_command_ids_only_shrink](unmapped_command_ids_only_shrink.md) — Coverage ratchet: the set of catalog ids with no dispatch arm may

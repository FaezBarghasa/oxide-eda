# updates_dialog

## Classs

- [BumpKind](BumpKind.md) — Severity classification for a drift entry — drives the default
- [LibraryUpdateEntry](LibraryUpdateEntry.md) — One row in the modal — drift between a placed Symbol's pinned
- [LibraryUpdatesState](LibraryUpdatesState.md) — Modal state — owned by [`crate::library::LibraryState::library_updates`].

## Functions

- [classify_bump](classify_bump.md) — Classify a `(current, latest)` semver-style version pair into a
- [classify_bump_distinguishes_three_buckets](classify_bump_distinguishes_three_buckets.md) — `classify_bump` distinguishes patch / minor / major by the
- [default_checked](default_checked.md) — Default checkbox state for the "Update Selected Components"
- [default_checked](default_checked_1.md) — Default checkbox state for the "Update Selected Components"
- [default_checked_only_patches](default_checked_only_patches.md) — Default checkbox follows the bump kind: patch on, others off.
- [label](label.md) — Short label for the badge column.
- [label](label_1.md) — Short label for the badge column.
- [new](new.md) — Build a state from a freshly-collected drift list. Auto-sorts
- [new](new_1.md) — Build a state from a freshly-collected drift list. Auto-sorts
- [new_sorts_by_ref_des_and_pre_checks_patches](new_sorts_by_ref_des_and_pre_checks_patches.md) — `LibraryUpdatesState::new` sorts by `ref_des` and applies the
- [parts](parts.md)
- [primary_btn](primary_btn.md)
- [render_entry_row](render_entry_row.md)
- [secondary_btn](secondary_btn.md)
- [selected_count](selected_count.md) — Number of entries the user has currently checked.
- [selected_count](selected_count_1.md) — Number of entries the user has currently checked.
- [toggle](toggle.md) — Toggle the checkbox for one entry, addressed by `symbol_uuid`.
- [toggle](toggle_1.md) — Toggle the checkbox for one entry, addressed by `symbol_uuid`.
- [toggle_flips_only_matching_uuid](toggle_flips_only_matching_uuid.md) — `toggle` flips the checkbox for a matching `symbol_uuid` and
- [view](view.md) — Render the modal card. Returns an `Element<LibraryMessage>` so the

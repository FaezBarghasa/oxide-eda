# mod

## Classs

- [AbColors](AbColors.md)
- [ActiveBarAction](ActiveBarAction.md) — All actions available from Active Bar buttons and dropdown items.
- [ActiveBarMenu](ActiveBarMenu.md) — Which Active Bar dropdown menu is open (by button index).
- [ActiveBarMsg](ActiveBarMsg.md) — [derive(Debug, Clone)]
- [CustomFilterPreset](CustomFilterPreset.md) — A user-defined named selection-filter preset. Persisted to
- [FootprintFilterPreset](FootprintFilterPreset.md) — A footprint-editor selection-filter preset. Parallel to
- [SelectionFilter](SelectionFilter.md) — Selection filter categories — each can be independently toggled.

## Functions

- [action_enabled](action_enabled.md) — Whether `action` is clickable given the current selection and
- [action_icon](action_icon.md) — Resolve the toolbar icon for the last-used action in a group.
- [as_set](as_set.md) — Realize the preset's `Vec` into a `HashSet` for assignment back
- [as_set](as_set_1.md) — Realize the preset's `Vec` into a `HashSet` for assignment back
- [capture](capture.md) — Snapshot the active filter set into a new preset with a default name.
- [capture](capture_1.md) — Snapshot the active filter set into a new preset with a default name.
- [from_tokens](from_tokens.md)
- [from_tokens](from_tokens_1.md)
- [label](label.md)
- [label](label_1.md)
- [requires_net_color](requires_net_color.md) — Whether `action` only makes sense when at least one net carries a
- [requires_selection](requires_selection.md) — Whether `action` needs at least one selected item to make sense.
- [view_bar](view_bar.md) — Render the Active Bar (the floating toolbar strip).

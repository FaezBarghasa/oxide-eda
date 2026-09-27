# mod

## Classs

- [ExportIssues](ExportIssues.md) — Everything that went wrong while deriving the export's netlist.

## Functions

- [build_export_context](build_export_context.md) — Assemble the export context, discarding the stitch issues — for the
- [build_export_scope](build_export_scope.md) — The export's page set + metadata + project netlist, alongside the stitch
- [export_metadata](export_metadata.md) — Title-block metadata for the export, plus the project's active variant.
- [log_stitch_issues](log_stitch_issues.md) — Put the stitch issues in front of the user — called once per *user action*
- [loose_pages](loose_pages.md) — Pages for a loose .snxsch: the unowned open tabs, active one first. An open
- [messages](messages.md) — One user-facing line per problem, in a stable order.
- [messages](messages_1.md) — One user-facing line per problem, in a stable order.
- [netlist_is_incomplete](netlist_is_incomplete.md) — Whether the derived netlist is missing connectivity rather than merely
- [netlist_is_incomplete](netlist_is_incomplete_1.md) — Whether the derived netlist is missing connectivity rather than merely
- [owned_pages](owned_pages.md) — One page per project sheet entry, in list order. Sheets open as tabs carry

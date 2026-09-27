# format

## Subdirectories

- [extras](extras/index.md)
- [mod](mod/index.md)
- [pcb_rows](pcb_rows/index.md)
- [sch_rows](sch_rows/index.md)
- [tests](tests/index.md)
- [tsv](tsv/index.md)
- [units](units/index.md)

## Modules

- [extras](extras.md) — Auxiliary "extras" sub-tables — the per-symbol / per-sheet and
- [format](mod.md) — Oxide native file formats — `.snxsch` (schematic) and `.snxpcb` (PCB).
- [pcb_rows](pcb_rows.md) — PCB bulk-row DTOs (`[footprints]` / `[pads]` / `[tracks]` /
- [sch_rows](sch_rows.md) — Schematic bulk-row DTOs (`[sheets.*]` TSV blocks) and their
- [tests](tests.md) — Round-trip / serialization tests for the `.snxsch` / `.snxpcb`
- [tsv](tsv.md) — TSV bulk-block codec: cell encode/decode, TOML-envelope escaping,
- [units](units.md) — Coordinate-unit helpers for the on-disk wire format.

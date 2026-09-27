# routes

## Subdirectories

- [error](error/index.md)
- [footprints](footprints/index.md)
- [locks](locks/index.md)
- [rows](rows/index.md)
- [sims](sims/index.md)
- [symbols](symbols/index.md)
- [tables](tables/index.md)

## Modules

- [error](error.md) — Shared `ApiError` envelope reused by every route module.
- [footprints](footprints.md) — `/footprints` routes — primitive CRUD mirror of `routes::symbols`.
- [locks](locks.md) — `/rows/:row_id/locks` — advisory locking over the row tier.
- [routes](mod.md) — HTTP route modules — split by resource.
- [rows](rows.md) — `/tables/:name/rows` routes — per-row CRUD over `ComponentRow`.
- [sims](sims.md) — `/sims` routes — primitive CRUD mirror of `routes::symbols`.
- [symbols](symbols.md) — `/symbols` routes — primitive CRUD for the v0.9 library refactor.
- [tables](tables.md) — `/tables` routes — DBLib row model.

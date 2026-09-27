# distributors

## Subdirectories

- [cache](cache/index.md)
- [digikey](digikey/index.md)
- [jlcpcb](jlcpcb/index.md)
- [keyring](keyring/index.md)
- [lcsc](lcsc/index.md)
- [mouser](mouser/index.md)

## Modules

- [cache](cache.md) — Disk JSON cache for `DistributorPart` records.
- [digikey](digikey.md) — DigiKey distributor adapter — OAuth2 PKCE scaffold.
- [distributors](mod.md) — Community distributor adapters (DigiKey, Mouser, LCSC, JLCPCB).
- [jlcpcb](jlcpcb.md) — JLCPCB distributor adapter — anonymous, polite-throttled (1 req/s).
- [keyring](keyring.md) — OS keyring credential storage for distributor adapters.
- [lcsc](lcsc.md) — LCSC distributor adapter — anonymous, polite-throttled (1 req/s).
- [mouser](mouser.md) — Mouser distributor adapter — API-key auth from OS keyring.

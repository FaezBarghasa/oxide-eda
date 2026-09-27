# distributor

## Classs

- [DistributorAdapter](DistributorAdapter.md)
- [DistributorError](DistributorError.md) — [derive(Debug, thiserror::Error)]
- [DistributorPart](DistributorPart.md) — [derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
- [DistributorSource](DistributorSource.md) — [derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
- [PriceBreak](PriceBreak.md) — Price-break tier for one distributor — `qty @ unit_price_usd`.
- [PricingSnapshot](PricingSnapshot.md) — Captured pricing snapshot for a single MPN, partitioned by distributor.

## Functions

- [_accepts_dyn](accepts_dyn.md)
- [distributor_adapter_is_object_safe](distributor_adapter_is_object_safe.md) — [test]
- [distributor_part_round_trip](distributor_part_round_trip.md) — [test]

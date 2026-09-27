# types

## Classs

- [ComponentHarvester](ComponentHarvester.md) — Component harvesting contract.
- [DiscoveredPin](DiscoveredPin.md) — Discovered pin metadata extracted from unstructured datasheets or structured APIs.
- [ElectricalPinType](ElectricalPinType.md) — Electrical role of a pin discovered from datasheets or distributor APIs.
- [HarvestedRawData](HarvestedRawData.md) — Raw harvested data payload from distributor APIs, crawlers, or OCR ingestion.
- [HarvestError](HarvestError.md) — [derive(Error, Debug)]
- [HarvestQuery](HarvestQuery.md) — [derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
- [PackageDimensions](PackageDimensions.md) — Mechanical package dimensions extracted or synthesized for IPC-7351C footprint generation.

## Functions

- [from_pin_name](from_pin_name.md) — Infers electrical pin type from pin name / token heuristics.
- [from_pin_name](from_pin_name_1.md) — Infers electrical pin type from pin name / token heuristics.
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [standard_chip](standard_chip.md) — Creates standard 2-terminal SMD chip dimensions (e.g., 0402, 0603, 0805, 1206).
- [standard_chip](standard_chip_1.md) — Creates standard 2-terminal SMD chip dimensions (e.g., 0402, 0603, 0805, 1206).
- [standard_qfn](standard_qfn.md) — Creates standard QFN dimensions.
- [standard_qfn](standard_qfn_1.md) — Creates standard QFN dimensions.
- [standard_soic](standard_soic.md) — Creates default standard SOIC dimensions for standard pin count.
- [standard_soic](standard_soic_1.md) — Creates default standard SOIC dimensions for standard pin count.
- [with_manufacturer](with_manufacturer.md)
- [with_manufacturer](with_manufacturer_1.md)
- [with_package](with_package.md)
- [with_package](with_package_1.md)
- [with_type](with_type.md)
- [with_type](with_type_1.md)

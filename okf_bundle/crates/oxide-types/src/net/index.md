# net

## Classs

- [DiffPair](DiffPair.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [Net](Net.md) — A logical net: a set of electrically-connected terminals derived from the
- [NetClass](NetClass.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [NetClassId](NetClassId.md) — [derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
- [NetId](NetId.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
- [Netlist](Netlist.md) — The authoritative netlist: every net derived from a schematic. This is the
- [Terminal](Terminal.md) — One pin instance connected to a net: the placed symbol's `uuid`, its
- [XSignal](XSignal.md) — An xSignal represents a complete physical/logical signal path spanning across
- [XSignalNode](XSignalNode.md) — A node in an extended signal (xSignal) flight path.

## Functions

- [from_nets](from_nets.md)
- [from_nets](from_nets_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [package_delay_to_length_microns](package_delay_to_length_microns.md) — Convert internal package delay into equivalent copper track length in micrometers
- [package_delay_to_length_microns](package_delay_to_length_microns_1.md) — Convert internal package delay into equivalent copper track length in micrometers
- [total_package_delay_ps](total_package_delay_ps.md) — Total internal package delay in picoseconds (ps) from source and destination pins.
- [total_package_delay_ps](total_package_delay_ps_1.md) — Total internal package delay in picoseconds (ps) from source and destination pins.

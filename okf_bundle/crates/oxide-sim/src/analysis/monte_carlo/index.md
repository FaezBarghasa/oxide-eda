# monte_carlo

## Classs

- [DeterministicPrng](DeterministicPrng.md) — Linear congruential / XorShift64 deterministic PRNG for reproducible runs.
- [MonteCarloEngine](MonteCarloEngine.md) — Monte Carlo Trial Generator.
- [MonteCarloRun](MonteCarloRun.md) — A single Monte Carlo trial run definition.
- [ParameterDistribution](ParameterDistribution.md) — Statistical Distribution Kind.
- [TolerancedParameter](TolerancedParameter.md) — A component parameter subject to statistical Monte Carlo variations.

## Functions

- [generate_runs](generate_runs.md) — Generates $N$ reproducible parameter variation sets using a fixed seed.
- [generate_runs](generate_runs_1.md) — Generates $N$ reproducible parameter variation sets using a fixed seed.
- [new](new.md)
- [new](new_1.md)
- [next_f64](next_f64.md) — Generates uniform float in [0.0, 1.0).
- [next_f64](next_f64_1.md) — Generates uniform float in [0.0, 1.0).
- [next_gaussian](next_gaussian.md) — Generates standard normal sample via Box-Muller transform: N(0, 1).
- [next_gaussian](next_gaussian_1.md) — Generates standard normal sample via Box-Muller transform: N(0, 1).
- [next_u64](next_u64.md) — Generates next 64-bit random word.
- [next_u64](next_u64_1.md) — Generates next 64-bit random word.
- [test_deterministic_monte_carlo_reproducibility](test_deterministic_monte_carlo_reproducibility.md) — [test]

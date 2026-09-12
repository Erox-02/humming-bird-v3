from hbp100 import HBP100
import json
import time
import statistics

EXTRACTORS = [
    {
        "name": "Name",
        "entity_type": "NAME",
        "pattern": r"\b[A-Z][a-z]+ [A-Z][a-z]+\b",
        "confidence": 0.90,
    },
    {
        "name": "MRN",
        "entity_type": "ID",
        "pattern": r"\b\d{6}\b",
        "confidence": 0.95,
    },
    {
        "name": "Phone",
        "entity_type": "PHONE",
        "pattern": r"\b\d{10}\b",
        "confidence": 0.90,
    },
]

engine = HBP100()
for cfg in EXTRACTORS:
    engine.add_extractor(json.dumps(cfg))

TEXT = "Patient John Doe, MRN: 123456, Phone: 9876543210"

WARMUP = 1_000
ITERATIONS = 100_000
ROUNDS = 5

for _ in range(WARMUP):
    engine.process(TEXT)

print("Warm-up complete.")
print(f"Benchmark: {ITERATIONS:,} iterations × {ROUNDS} rounds\n")

results_ns = []

for round_no in range(ROUNDS):
    start = time.perf_counter_ns()

    for _ in range(ITERATIONS):
        engine.process(TEXT)

    elapsed_ns = time.perf_counter_ns() - start
    avg_ns = elapsed_ns / ITERATIONS

    results_ns.append(avg_ns)

    print(
        f"Round {round_no + 1}: "
        f"{elapsed_ns / 1e9:.4f}s | "
        f"{avg_ns / 1_000:.3f} µs/text | "
        f"{1e9 / avg_ns:,.0f} texts/sec"
    )

mean_ns = statistics.mean(results_ns)
median_ns = statistics.median(results_ns)
min_ns = min(results_ns)

sorted_results = sorted(results_ns)

p95_ns = sorted_results[int(len(sorted_results) * 0.95) - 1]
p99_ns = sorted_results[int(len(sorted_results) * 0.99) - 1]

print("\n" + "=" * 60)
print("HBP100 BENCHMARK")
print("=" * 60)

print(f"Mean:       {mean_ns / 1_000:.3f} µs/text")
print(f"Median:     {median_ns / 1_000:.3f} µs/text")
print(f"Minimum:    {min_ns / 1_000:.3f} µs/text")
print(f"P95:        {p95_ns / 1_000:.3f} µs/text")
print(f"P99:        {p99_ns / 1_000:.3f} µs/text")

print(f"\nThroughput: {1e9 / mean_ns:,.0f} texts/sec")

print("=" * 60)
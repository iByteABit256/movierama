import json
import numpy as np
import matplotlib.pyplot as plt

def parse_k6_json(file_path):
    """Extract http_req_duration and http_reqs metrics from k6 JSON output."""
    durations = []
    reqs = 0

    with open(file_path, "r") as f:
        for line in f:
            try:
                data = json.loads(line)
            except json.JSONDecodeError:
                continue

            if data.get("type") != "Point":
                continue

            metric = data.get("metric")
            value = data.get("data", {}).get("value")

            if metric == "http_req_duration" and isinstance(value, (int, float)):
                durations.append(value)
            elif metric == "http_reqs" and isinstance(value, (int, float)):
                reqs += value

    return durations, reqs


def summarize(durations, reqs, test_duration_s):
    """Compute summary metrics for latency and throughput."""
    if not durations:
        return {"avg": 0, "p90": 0, "p99": 0, "reqs_per_sec": 0}

    avg = np.mean(durations)
    p90 = np.percentile(durations, 90)
    p99 = np.percentile(durations, 99)
    rps = reqs / test_duration_s

    return {"avg": avg, "p90": p90, "p99": p99, "reqs_per_sec": rps}


# --- Main ---
rust_durations, rust_reqs = parse_k6_json("rust_results.json")
spring_durations, spring_reqs = parse_k6_json("spring_results.json")

# Adjust based on your k6 test duration (in seconds)
TEST_DURATION_S = 30

rust_summary = summarize(rust_durations, rust_reqs, TEST_DURATION_S)
spring_summary = summarize(spring_durations, spring_reqs, TEST_DURATION_S)

print("Rust Summary:", rust_summary)
print("Spring Summary:", spring_summary)

# Plot
labels = ["Avg (ms)", "P90 (ms)", "P99 (ms)", "Req/s"]
rust_values = [
    rust_summary["avg"],
    rust_summary["p90"],
    rust_summary["p99"],
    rust_summary["reqs_per_sec"],
]
spring_values = [
    spring_summary["avg"],
    spring_summary["p90"],
    spring_summary["p99"],
    spring_summary["reqs_per_sec"],
]

x = np.arange(len(labels))
width = 0.35

fig, ax = plt.subplots()
ax.bar(x - width / 2, rust_values, width, label="Rust", color="#dea584")
ax.bar(x + width / 2, spring_values, width, label="Spring Boot", color="#5382a1")

ax.set_ylabel("Value")
ax.set_title("Rust vs Spring Boot — Performance Comparison")
ax.set_xticks(x)
ax.set_xticklabels(labels)
ax.legend()

plt.tight_layout()
plt.savefig("comparison.png", dpi=200)
print("✅ Saved chart as comparison.png")


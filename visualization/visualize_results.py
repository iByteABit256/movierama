import json
import numpy as np
import matplotlib.pyplot as plt
import pandas as pd


# --- Parse k6 JSON results ---
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


# --- Parse docker_stats.csv ---
def parse_docker_stats(file_path):
    """Parse docker_stats.csv and convert CPU and memory usage to numeric."""
    df = pd.read_csv(file_path)
    df["timestamp"] -= df["timestamp"].min()  # normalize to 0 start
    df["cpu_perc"] = df["cpu_perc"].str.replace("%", "").astype(float)

    # Parse memory like "65.3MiB / 7.77GiB"
    def parse_mem_usage(mem_str):
        used = mem_str.split("/")[0].strip()
        if "GiB" in used:
            return float(used.replace("GiB", "").strip()) * 1024
        elif "MiB" in used:
            return float(used.replace("MiB", "").strip())
        elif "KiB" in used:
            return float(used.replace("KiB", "").strip()) / 1024
        else:
            return float(used)

    df["mem_mib"] = df["mem_usage"].apply(parse_mem_usage)
    return df


# --- Main ---
rust_durations, rust_reqs = parse_k6_json("rust_results.json")
spring_durations, spring_reqs = parse_k6_json("spring_results.json")

# Adjust test duration (in seconds)
TEST_DURATION_S = 30

rust_summary = summarize(rust_durations, rust_reqs, TEST_DURATION_S)
spring_summary = summarize(spring_durations, spring_reqs, TEST_DURATION_S)

print("Rust Summary:", rust_summary)
print("Spring Summary:", spring_summary)

# --- Plot latency comparison ---
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

fig, axes = plt.subplots(3, 1, figsize=(10, 12))
ax1, ax2, ax3 = axes

# --- Chart 1: Latency comparison ---
ax1.bar(x - width / 2, rust_values, width, label="Rust", color="#dea584")
ax1.bar(x + width / 2, spring_values, width, label="Spring Boot", color="#5382a1")
ax1.set_ylabel("Value (ms or req/s)")
ax1.set_title("Rust vs Spring Boot — Performance Comparison")
ax1.set_xticks(x)
ax1.set_xticklabels(labels)
ax1.legend()

# --- Chart 2 & 3: Docker CPU + Memory ---
try:
    rust_stats = parse_docker_stats("rust_docker_stats.csv")
    spring_stats = parse_docker_stats("spring_docker_stats.csv")

    # CPU usage comparison
    ax2.plot(rust_stats["timestamp"], rust_stats["cpu_perc"], label="Rust", color="#dea584")
    ax2.plot(spring_stats["timestamp"], spring_stats["cpu_perc"], label="Spring Boot", color="#5382a1")
    ax2.set_title("CPU Usage Over Time")
    ax2.set_xlabel("Time (s)")
    ax2.set_ylabel("CPU %")
    ax2.legend()

    # Memory usage comparison
    ax3.plot(rust_stats["timestamp"], rust_stats["mem_mib"], label="Rust", color="#dea584")
    ax3.plot(spring_stats["timestamp"], spring_stats["mem_mib"], label="Spring Boot", color="#5382a1")
    ax3.set_title("Memory Usage Over Time")
    ax3.set_xlabel("Time (s)")
    ax3.set_ylabel("Memory (MiB)")
    ax3.legend()

except FileNotFoundError:
    print("⚠️ Skipping Docker resource plots — rust_docker_stats.csv or spring_docker_stats.csv missing")

plt.tight_layout()
plt.savefig("comparison_dashboard.png", dpi=200)
print("✅ Saved full performance dashboard as comparison_dashboard.png")

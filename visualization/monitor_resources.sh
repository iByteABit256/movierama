#!/usr/bin/env bash
# Monitor backend resource usage during k6 test

CONTAINER_NAME="movierama-backend"
OUTPUT_FILE="rust_docker_stats.csv"

echo "timestamp,cpu_perc,mem_usage,mem_perc" > "$OUTPUT_FILE"

while true; do
  docker stats --no-stream --format \
    "{{.CPUPerc}},{{.MemUsage}},{{.MemPerc}}" "$CONTAINER_NAME" | \
    awk -v date="$(date +%s)" '{print date "," $0}' >> "$OUTPUT_FILE"
  sleep 1
done


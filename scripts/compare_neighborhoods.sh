#!/usr/bin/env bash
set -euo pipefail

INSTANCES=(
    "instances/favela-dos-anjos-922/graph"
    "instances/jardim-centenario-1117/graph"
)
ITERATIONS=100
EXECUTIONS=5
META="grasp"
BINARY="./target/release/main"
COMMON_ARGS="--metaheuristic $META --iterations $ITERATIONS --executions $EXECUTIONS -q"

ALL="expand-route,add-loop,reposition-unit"

declare -A CONFIGS
CONFIGS["all"]="$ALL"
CONFIGS["no-expand"]="add-loop,reposition-unit"
CONFIGS["no-loop"]="expand-route,reposition-unit"
CONFIGS["no-reposition"]="expand-route,add-loop"

cargo build --release --bin main 2>&1

for instance in "${INSTANCES[@]}"; do
    instance_name=$(basename "$(dirname "$instance")")
    printf "\n=== %s ===\n\n" "$instance_name"
    printf "%-20s %s\n" "CONFIG" "SCORE"
    printf "%-20s %s\n" "------" "-----"

    for name in all no-expand no-loop no-reposition; do
        neighborhoods="${CONFIGS[$name]}"
        score=$($BINARY "$instance" $COMMON_ARGS --neighborhoods "$neighborhoods" --irace)
        printf "%-20s %s\n" "$name" "$score"
    done
done

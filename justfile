mine file out:
    ./mining/seqwog -tm {{file}} {{out}}

report-greedy:
    cargo run --release --bin=report -- \
        2024-02-06-one-greedy-calibrado-com-fixed-unit-gulosa 10 1 \
        --iterations=1 \
        --metaheuristic=one-greedy-construction \
        --alpha=1 \
        --max-edges-between-loop=0.2620 \
        --max-section-size=0.3719 \
        --fixed-unit-strategy=greedy
# Experimentos

## setup

linguagem, compilador, máquina, explicação das instâncias

## experimentos

# Report only optima
(
    cargo run --release --bin report relatorio-20-05 10 1000 1 --only-optima --alpha=0.3 --max-section-size=0.3 --max-edges-between-loop=0.3


    cargo run --release --bin report relatorio-20-05 10 1000 1 --only-optima --alpha=0.6 --max-section-size=0.3 --max-edges-between-loop=0.3
)

fixar iteração em 100, variar alfa, max-section-size --max...-loop

alpha: [0; 1)
intervalo --max: [0.1 - 0.4]

"completo-492"
"vila-progresso-1988"
"vila-formosa-3460"

rodar instancia pequenas

comparar nosso resultado com modelo

Colocar o resultado das grandes (que o modelo não roda)
    Usar os parâmetros default


Comparar com a do wagner x Heurística

## TODO

- Rodar report para as instãncias grandes
  - com os parâmetros novos calibrados com 1000 experimentos no irace
  - com os parâmetros default

Comando rodado:
cargo run --release --bin report -- \
2022-09-04-full-parametros-default 10 100 1 \
&& \
cargo run --release --bin report -- \
2022-09-04-full-parametros-calibrados 10 100 1 \
--alpha=0.1436 \
--max-edges-between-loop=0.2620 \
--max-section-size=0.3719 \
&& \
shutdown


cargo run --release -- \
2022-09-04-full-parametros-calibrados 10 100 1 \
--alpha=0.1436 \
--max-edges-between-loop=0.2620 \
--max-section-size=0.3719
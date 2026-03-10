# Gerador de arquivos DOT

Essa ferramenta tem como intuito gerar arquivos DOT com base em instâncias e soluções
do problema de cobertura de crime para facilitar a análise dos resultados obtidos.

## Como Usar

OBS: Os exemplos a seguir se baseiam em um sistema GNU/Linux.

Executando o comando a partir da raiz do projeto, temos o seguinte _template_:

`python3 dot/dot.py <arquivo de entrada do problema> <solução> [solução...]`

Caso não haja nenhum problema, o nome do arquivo será `<nome do arquivo de entrada>.dot`.

Exemplo de uso para comparar as soluções heurística e ótima na instância mini3:

`python3 dot/dot.py mini3.txt resultado-mini3.txt mini3.sol`

O arquivos de saída, nesse caso, se chamará `mini3.txt.dot`.

Com o arquivo dot, basta passá-lo a algum utilitário do graphviz para gerar a imagem. Exemplo:

`dot -Tpng -o mini3.png mini3.txt.dot`

Usando o programa `dot` (obtido ao instalar o graphviz), o comando acima gera uma imagem no formato png, com nome de saída `mini3.png` com base no arquivo `mini3.txt.dot`.

## Problemas Conhecidos

A ferramenta espera que, se existe uma linha para uma rota, deve haver algum elemento para essa rota. Isso quer dizer que o seguinte trecho não seria válido para a ferramenta:

```
Zona 2:
    1)
    2)
```

Pois indica que a zona 2 possui 2 rotas (`1); 2)`) mas ambas estão vazias. A saída da heurística não imprime isso, mas há algumas soluções ótimas que fazem isso. Um ´workaround´ rápido é abrir o arquivo e apagar essas linhas vazias.

A solução ótima exibe o texto `(*ÓTIMO*)` ao lado do valor de solução, na primeira linha de seu arquivo. A ferramenta não está usando o valor de solução para nada atualmente, mas, caso use, esse texto pode gerar um conflito.

Atualmente, a ferramenta espera que haja no máximo 3 soluções para uma instãncia, mas o ajuste para permitir mais é bem simples (adicionar mais cores no `graph_colors`, no começo do `exporters.py`).

## Aprimoramentos

Fique à vontade para abrir _issues_ no repositório para esclarecer dúvidas não cobertas nesse documento ou para debater sobre ajustes na ferramenta.

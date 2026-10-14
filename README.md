# ConectaStore MegaStore

Sistema demonstrativo de recomendação de produtos em Rust. O grafo conecta clientes, produtos e categorias por compras, interesses, pertencimento e similaridade. A consulta devolve produtos disponíveis, ordenados pela pontuação, sem itens repetidos.

## Requisitos e execução

- Rust com Cargo (edição 2021; recomenda-se uma versão recente).
- O projeto utiliza apenas a biblioteca padrão, sem dependências externas.

Na pasta `megastore`:

```bash
cargo run --release -- demo
cargo test
cargo run --release -- bench
```

O primeiro comando mostra recomendações para o cliente 1 e produtos relacionados ao notebook 101. O segundo executa testes unitários e de integração. O terceiro mede a construção e 100 consultas em cada volume de dados.

Exemplo esperado na demonstração (ordem definida por pontos e depois ID):

```text
Recomendacoes para o cliente 1:
102: Mouse (5 pontos)
103: Teclado (5 pontos)
Relacionados ao produto 101:
102: Mouse (3 pontos)
103: Teclado (1 pontos)
```

## Arquitetura

| Arquivo | Responsabilidade |
| --- | --- |
| `src/catalogo.rs` | Cadastro, atualização e consulta de produtos por ID em `HashMap`. |
| `src/grafo.rs` | Arestas bidirecionais em listas de adjacência `HashMap<No, HashSet<No>>`; índices de compras, interesses e categorias; BFS. |
| `src/recomendacao.rs` | Formação, pontuação, filtro e ordenação dos candidatos. |
| `src/main.rs` | Demonstração e medição de desempenho. |
| `tests/integracao.rs` | Cenários completos de recomendação. |

`No` distingue `Cliente`, `Produto` e `Categoria`, mesmo quando possuem o mesmo ID numérico. As relações são armazenadas nos dois sentidos para permitir navegação a partir de um cliente ou produto. `HashSet` impede arestas duplicadas, e um `HashMap` de pontuação consolida cada produto em uma única recomendação.

### Algoritmo

A BFS (busca em largura) usa `VecDeque` e limita o percurso a dois saltos. Ela encontra vizinhos diretos de um produto e itens ligados por uma categoria, ou candidatos alcançados a partir do cliente. O limite impede percorrer indiscriminadamente todo o grafo. Na recomendação por cliente, um produto alcançado a dois saltos recebe 3 pontos e a presença numa categoria de interesse acrescenta 2. Na recomendação por produto, um vizinho direto recebe 3 pontos e um produto a dois saltos recebe 1. Compras anteriores, produtos indisponíveis e o próprio produto de origem são filtrados. Empates são resolvidos pelo menor ID.

Esses pesos são didáticos. Avaliações e recência podem ser adicionadas como relações e sinais de pontuação. Uma implementação de produção precisaria de persistência, atualização incremental e limites por grau, porque uma categoria muito grande pode alcançar muitos candidatos em apenas dois saltos.

O armazenamento por adjacência custa `O(V + E)` para vértices `V` e arestas `E`. Uma BFS limitada custa `O(V_visitados + E_visitadas)` no subgrafo alcançado. Se `C` candidatos forem encontrados, ordená-los custa `O(C log C)`. Consultas médias aos índices hash são `O(1)`; seu pior caso depende de colisões e implementação.

## Testes

Há testes unitários para cadastro, atualização, BFS e prevenção de arestas duplicadas. Os testes de integração verificam recomendação por cliente e produto, exclusão de compras, disponibilidade, empates, limite `k` e IDs inexistentes. Execute `cargo test` e preserve o resultado do terminal como evidência da entrega.

## Desempenho

Execute `cargo run --release -- bench` no computador usado para a apresentação. O programa constrói grafos com 1.000, 10.000 e 100.000 produtos, com 100 categorias e relações de similaridade em cadeia. Para cada volume, imprime tempo de construção, tempo de 100 recomendações e média por consulta em milissegundos.

| Produtos | Construção | 100 consultas | Média por consulta |
| ---: | ---: | ---: | ---: |
| 1.000 | A preencher após execução | A preencher | A preencher |
| 10.000 | A preencher após execução | A preencher | A preencher |
| 100.000 | A preencher após execução | A preencher | A preencher |

Os tempos dependem do hardware, da versão do Rust e da carga do computador. Este ambiente de elaboração não tinha `cargo` ou `rustc`, portanto não foram inventados resultados. A medição usa `std::time::Instant` e mede a construção separadamente das consultas; não substitui um benchmark estatístico com aquecimento e múltiplas amostras independentes.

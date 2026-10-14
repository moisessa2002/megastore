use std::collections::{HashMap, HashSet};
use crate::{Catalogo, Grafo, No};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recomendacao { pub produto_id: u32, pub pontos: u32 }

pub struct Recomendador<'a> { pub catalogo: &'a Catalogo, pub grafo: &'a Grafo }

impl Recomendador<'_> {
    fn finalizar(&self, pontos: HashMap<u32, u32>, excluir: &HashSet<u32>, k: usize) -> Vec<Recomendacao> {
        let mut itens: Vec<_> = pontos.into_iter()
            .filter(|(id, _)| !excluir.contains(id)
                && self.catalogo.consultar(*id).is_some_and(|p| p.disponivel))
            .map(|(produto_id, pontos)| Recomendacao { produto_id, pontos }).collect();
        itens.sort_by(|a, b| b.pontos.cmp(&a.pontos).then(a.produto_id.cmp(&b.produto_id)));
        itens.truncate(k);
        itens
    }

    pub fn para_cliente(&self, cliente: u32, k: usize) -> Vec<Recomendacao> {
        let comprados = self.grafo.comprados(cliente).cloned().unwrap_or_default();
        let mut pontos: HashMap<u32, u32> = HashMap::new();
        // BFS de até dois saltos alcança compras, categorias de interesse e similares.
        for (no, distancia) in self.grafo.bfs(No::Cliente(cliente), 2) {
            if let No::Produto(id) = no {
                if distancia == 2 && !comprados.contains(&id) {
                    // Um produto pode chegar por mais de um caminho. HashMap consolida o candidato.
                    *pontos.entry(id).or_default() += 3;
                }
            }
        }
        // O índice de categorias acrescenta evidência sem repetir os produtos no resultado.
        if let Some(interesses) = self.grafo.interesses(cliente) {
            for &categoria in interesses {
                if let Some(ids) = self.grafo.na_categoria(categoria) {
                    for &id in ids { *pontos.entry(id).or_default() += 2; }
                }
            }
        }
        self.finalizar(pontos, &comprados, k)
    }

    pub fn para_produto(&self, produto: u32, k: usize) -> Vec<Recomendacao> {
        if self.catalogo.consultar(produto).is_none() { return Vec::new(); }
        let mut pontos = HashMap::new();
        for (no, distancia) in self.grafo.bfs(No::Produto(produto), 2) {
            if let No::Produto(id) = no {
                if id != produto && (distancia == 1 || distancia == 2) {
                    pontos.insert(id, if distancia == 1 { 3 } else { 1 });
                }
            }
        }
        self.finalizar(pontos, &HashSet::from([produto]), k)
    }
}

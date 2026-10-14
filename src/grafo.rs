use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum No { Cliente(u32), Produto(u32), Categoria(u32) }

#[derive(Default)]
pub struct Grafo {
    adj: HashMap<No, HashSet<No>>,
    compras: HashMap<u32, HashSet<u32>>,
    interesses: HashMap<u32, HashSet<u32>>,
    categorias: HashMap<u32, HashSet<u32>>,
}

impl Grafo {
    pub fn novo() -> Self { Self::default() }
    fn ligar(&mut self, a: No, b: No) {
        self.adj.entry(a).or_default().insert(b);
        self.adj.entry(b).or_default().insert(a);
    }
    pub fn produto_na_categoria(&mut self, produto: u32, categoria: u32) {
        // A categoria de um produto é imutável depois do cadastro no cenário de demonstração.
        self.categorias.entry(categoria).or_default().insert(produto);
        self.ligar(No::Produto(produto), No::Categoria(categoria));
    }
    pub fn registrar_compra(&mut self, cliente: u32, produto: u32) {
        self.compras.entry(cliente).or_default().insert(produto);
        self.ligar(No::Cliente(cliente), No::Produto(produto));
    }
    pub fn registrar_interesse(&mut self, cliente: u32, categoria: u32) {
        self.interesses.entry(cliente).or_default().insert(categoria);
        self.ligar(No::Cliente(cliente), No::Categoria(categoria));
    }
    pub fn similares(&mut self, a: u32, b: u32) {
        if a != b { self.ligar(No::Produto(a), No::Produto(b)); }
    }
    pub fn comprados(&self, cliente: u32) -> Option<&HashSet<u32>> { self.compras.get(&cliente) }
    pub fn interesses(&self, cliente: u32) -> Option<&HashSet<u32>> { self.interesses.get(&cliente) }
    pub fn na_categoria(&self, categoria: u32) -> Option<&HashSet<u32>> { self.categorias.get(&categoria) }

    /// BFS até max_saltos. Retorna a menor distância de cada nó alcançado.
    pub fn bfs(&self, origem: No, max_saltos: usize) -> HashMap<No, usize> {
        let mut distancias = HashMap::from([(origem, 0)]);
        let mut fila = VecDeque::from([origem]);
        while let Some(atual) = fila.pop_front() {
            let distancia = distancias[&atual];
            if distancia >= max_saltos { continue; }
            if let Some(vizinhos) = self.adj.get(&atual) {
                for &vizinho in vizinhos {
                    if let std::collections::hash_map::Entry::Vacant(e) = distancias.entry(vizinho) {
                        e.insert(distancia + 1);
                        fila.push_back(vizinho);
                    }
                }
            }
        }
        distancias
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bfs_encontra_menor_distancia_e_respeita_limite() {
        let mut g = Grafo::novo();
        g.registrar_compra(1, 10);
        g.similares(10, 11);
        assert_eq!(g.bfs(No::Cliente(1), 1).get(&No::Produto(11)), None);
        assert_eq!(g.bfs(No::Cliente(1), 2).get(&No::Produto(11)), Some(&2));
    }
    #[test]
    fn arestas_duplicadas_nao_aumentam_vizinhanca() {
        let mut g = Grafo::novo();
        g.similares(1, 2); g.similares(1, 2);
        assert_eq!(g.bfs(No::Produto(1), 1).len(), 2);
    }
}

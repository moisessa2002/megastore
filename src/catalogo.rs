use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: u32,
    pub disponivel: bool,
}

impl Produto {
    pub fn novo(id: u32, nome: impl Into<String>, categoria: u32) -> Self {
        Self { id, nome: nome.into(), categoria, disponivel: true }
    }
}

#[derive(Default)]
pub struct Catalogo {
    produtos: HashMap<u32, Produto>,
}

impl Catalogo {
    pub fn novo() -> Self { Self::default() }
    /// Retorna true quando um produto existente foi substituído.
    pub fn cadastrar(&mut self, produto: Produto) -> bool {
        self.produtos.insert(produto.id, produto).is_some()
    }
    pub fn consultar(&self, id: u32) -> Option<&Produto> { self.produtos.get(&id) }
    pub fn consultar_mut(&mut self, id: u32) -> Option<&mut Produto> { self.produtos.get_mut(&id) }
    pub fn total(&self) -> usize { self.produtos.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cadastra_consulta_e_atualiza() {
        let mut c = Catalogo::novo();
        assert!(!c.cadastrar(Produto::novo(1, "Mouse", 10)));
        assert_eq!(c.consultar(1).unwrap().nome, "Mouse");
        assert!(c.cadastrar(Produto::novo(1, "Mouse sem fio", 10)));
        assert_eq!(c.total(), 1);
        assert_eq!(c.consultar(1).unwrap().nome, "Mouse sem fio");
        assert!(c.consultar(2).is_none());
    }
}

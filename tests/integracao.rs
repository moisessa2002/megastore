use megastore::{Catalogo, Grafo, Produto, Recomendador};

#[test]
fn recomenda_por_cliente_sem_duplicar_nem_repetir_compra() {
    let mut c = Catalogo::novo(); let mut g = Grafo::novo();
    for id in 1..=4 {
        c.cadastrar(Produto::novo(id, format!("Item {id}"), 10));
        g.produto_na_categoria(id, 10);
    }
    g.registrar_compra(7, 1);
    g.registrar_interesse(7, 10);
    g.similares(1, 2);
    g.similares(1, 2);
    c.consultar_mut(4).unwrap().disponivel = false;
    let r = Recomendador { catalogo: &c, grafo: &g };
    let ids: Vec<_> = r.para_cliente(7, 10).iter().map(|x| x.produto_id).collect();
    assert_eq!(ids, vec![2, 3]);
    assert_eq!(r.para_cliente(7, 10)[0].pontos, 5);
    assert_eq!(r.para_cliente(7, 1).len(), 1);
}

#[test]
fn recomenda_por_produto_e_lida_com_inexistente() {
    let mut c = Catalogo::novo(); let mut g = Grafo::novo();
    for id in 1..=3 {
        c.cadastrar(Produto::novo(id, format!("Item {id}"), 20));
        g.produto_na_categoria(id, 20);
    }
    g.similares(1, 2);
    let r = Recomendador { catalogo: &c, grafo: &g };
    let ids: Vec<_> = r.para_produto(1, 10).iter().map(|x| x.produto_id).collect();
    assert_eq!(ids, vec![2, 3]);
    assert!(r.para_produto(999, 10).is_empty());
    assert!(r.para_cliente(999, 10).is_empty());
}

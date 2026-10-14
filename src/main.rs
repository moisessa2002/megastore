use std::time::Instant;
use megastore::{Catalogo, Grafo, Produto, Recomendador};

fn exemplo() {
    let mut catalogo = Catalogo::novo();
    let mut grafo = Grafo::novo();
    for (id, nome, categoria) in [
        (101, "Notebook", 1), (102, "Mouse", 1),
        (103, "Teclado", 1), (201, "Livro Rust", 2),
    ] {
        catalogo.cadastrar(Produto::novo(id, nome, categoria));
        grafo.produto_na_categoria(id, categoria);
    }
    grafo.registrar_compra(1, 101);
    grafo.registrar_interesse(1, 1);
    grafo.similares(101, 102);
    let r = Recomendador { catalogo: &catalogo, grafo: &grafo };
    println!("Recomendacoes para o cliente 1:");
    for item in r.para_cliente(1, 5) {
        println!("{}: {} ({} pontos)", item.produto_id,
            catalogo.consultar(item.produto_id).unwrap().nome, item.pontos);
    }
    println!("Relacionados ao produto 101:");
    for item in r.para_produto(101, 5) {
        println!("{}: {} ({} pontos)", item.produto_id,
            catalogo.consultar(item.produto_id).unwrap().nome, item.pontos);
    }
}

fn medir(n: u32) {
    let mut catalogo = Catalogo::novo();
    let mut grafo = Grafo::novo();
    let inicio = Instant::now();
    for id in 1..=n {
        catalogo.cadastrar(Produto::novo(id, format!("Produto {id}"), id % 100));
        grafo.produto_na_categoria(id, id % 100);
        if id > 1 { grafo.similares(id - 1, id); }
    }
    grafo.registrar_compra(1, 1);
    grafo.registrar_interesse(1, 1);
    let construcao = inicio.elapsed();
    let r = Recomendador { catalogo: &catalogo, grafo: &grafo };
    let inicio = Instant::now();
    let mut quantidade = 0;
    for _ in 0..100 { quantidade = r.para_cliente(1, 10).len(); }
    let consultas = inicio.elapsed();
    println!("{n} produtos | construcao: {:.3} ms | 100 consultas: {:.3} ms | media: {:.3} ms | retornados: {quantidade}",
        construcao.as_secs_f64() * 1000., consultas.as_secs_f64() * 1000.,
        consultas.as_secs_f64() * 10.);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        None | Some("demo") => exemplo(),
        Some("bench") => {
            for n in [1_000, 10_000, 100_000] { medir(n); }
        }
        _ => {
            eprintln!("Uso: cargo run --release -- [demo|bench]");
            std::process::exit(2);
        }
    }
}

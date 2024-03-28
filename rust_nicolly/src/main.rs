use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct Matriz {
    data: Vec<Vec<i32>>,
}

fn main() {
    let matriz: Matriz = ler_matriz("../data/matriz.json");

    imprimir_matriz(&matriz.data);

    println!("Digite a operação desejada (M para média, S para soma): ");
    let operacao = ler_operacao();

    let numeros_selecionados = selecionar_numeros(&matriz.data);

    let resultado = match operacao.as_str() {
        "M" => {
            let soma: i32 = numeros_selecionados.iter().copied().sum();
            soma as f32 / numeros_selecionados.len() as f32
        }
        "S" => numeros_selecionados.iter().copied().sum::<i32>() as f32,
        _ => {
            println!("Opção inválida (M ou S)");
            return;
        }
    };
    

    println!("Resultado da operação: {}", resultado);

    imprimir_representacao_matriz();
    esperar_enter();
}

fn ler_matriz(path: &str) -> Matriz {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn imprimir_matriz(matriz: &Vec<Vec<i32>>) {
    for row in matriz {
        println!("{}", row.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(", "));
    }
}

fn ler_operacao() -> String {
    let mut operacao = String::new();
    stdin().read_line(&mut operacao).unwrap();
    operacao.trim().to_uppercase()
}

fn selecionar_numeros(matriz: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut numeros = Vec::new();
    let mut inicio = 7;
    let mut fim = 11;

    for linha in 0..matriz.len() {
        for coluna in inicio..=fim {
            numeros.push(matriz[linha][coluna]);
        }
        if inicio > 0 {
            inicio -= 1;
        }
        if fim < matriz[0].len() - 1 {
            fim += 1;
        }
    }
    numeros
}

fn imprimir_representacao_matriz() {
    for i in (0..6).rev() {
        print_dots_and_xs(i, 12);
    }
    for i in 6..12 {
        print_dots_and_xs(i, 12);
    }
}

fn print_dots_and_xs(chars: usize, max_chars: usize) {
    let num_dots = max_chars - chars;
    for _ in 0..num_dots {
        print!(". ");
    }
    for _ in 0..chars {
        print!("X ");
    }
    println!();
}

fn esperar_enter() {
    println!("Pressione Enter para sair...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
}

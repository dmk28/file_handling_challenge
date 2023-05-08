use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};

fn main() {
    // Abrindo o arquivo e lendo o conteúdo
    let file = File::open("dados.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        println!("{}", l);
    }

    // Adicionando novo conteúdo ao arquivo
    let mut file = OpenOptions::new()
        .append(true)
        .open("dados.txt")
        .unwrap();
    let novo_conteudo = "Nova linha de texto\n";
    file.write_all(novo_conteudo.as_bytes()).unwrap();

    // Contando a quantidade de linhas no arquivo
    let file = File::open("dados.txt").unwrap();
    let reader = BufReader::new(file);
    let num_lines = reader.lines().count();
    println!("Número de linhas: {}", num_lines);
}
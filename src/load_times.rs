// Ler o arquivo times.txt e retornar um vetor de strings com cada linha do arquivo.

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_times() -> Vec<String> {
    let file = File::open("times.txt").unwrap();
    let reader = BufReader::new(file);
    let mut times = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        times.push(line);
    }

    times
}
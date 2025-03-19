use regex::Regex;
use std::fs;

pub fn count_impl_blocks(file_path: &str) -> usize {
    // Legge il contenuto del file
    let content = fs::read_to_string(file_path).expect("Errore nella lettura del file");

    // Definisce una regex per trovare i blocchi `impl NomeStruct`
    let re = Regex::new(r"impl\s+\w+\s*\{").unwrap();

    // Conta quante corrispondenze ci sono nel file
    re.find_iter(&content).count()
}

pub fn count_struct_blocks(file_path: &str) -> usize {
    // Legge il contenuto del file
    let content = fs::read_to_string(file_path).expect("Errore nella lettura del file");

    // Definisce una regex per trovare i blocchi `impl NomeStruct`
    let re = Regex::new(r"struct\s+\w+\s*[{;]").unwrap();

    // Conta quante corrispondenze ci sono nel file
    re.find_iter(&content).count()
}


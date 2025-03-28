use std::fs;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;

// pub fn show_tree_methods(result: HashMap<String, HashMap<String, Vec<String>>>, path: &str) {
//     let mut methods = HashMap::new();
//     for (folder, files) in result.iter() {
//         for (file, functions) in files.iter() {
//             for _function in functions {
//                 let file_path = format!("{}\\src\\{}\\{}", path, folder, file);
//                 for (method_name, count) in project_analyzer::class_manager::count_methods(&file_path) {
//                     methods.entry(method_name).or_insert(count);
//                 }
//             }
//         }
//     }

//     // Riordinamento mappa per numero di chiamate
//     let mut method_count: Vec<_> = methods.into_iter().collect();
//     method_count.sort_by(|a, b| b.1.cmp(&a.1)); // Ordina in base al numero di chiamate (decrescente)


//     for (method_name, count) in method_count.iter() {
//         println!("Nome metodo: {} Chiamate: {}", method_name, count);
//     }
// }

pub fn count_methods(file_path: &str) -> usize {
    // Leggi il contenuto del file
    let mut file = File::open(file_path).expect("Impossibile aprire il file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Impossibile leggere il contenuto del file");

    // Crea una regex per trovare le dichiarazioni di metodi
    let re_fn = Regex::new(r"fn\s+([a-zA-Z0-9_]+)\s*(?:<[^>]+>)?\s*\(").unwrap();
    
    // Crea una regex per trovare le chiamate ai metodi
    let re_call = Regex::new(r"([a-zA-Z0-9_]+)\s*\(").unwrap();

    let mut method_count = HashMap::new();

    // Trova tutte le dichiarazioni di metodi e inizializza la mappa con 0 chiamate
    for cap in re_fn.captures_iter(&content) {
        let method_name = cap[1].to_string();
        println!("🔍 Funzione trovata: {}", &cap[1]);
        method_count.insert(method_name.clone(), 0);
    }

    // Conta quante volte ogni metodo viene chiamato
    for cap in re_call.captures_iter(&content) {
        let method_name = cap[1].to_string();
        if method_count.contains_key(&method_name) {
            let count = method_count.entry(method_name).or_insert(0);
            *count += 1;
        }
    }

    //Rimuove il main dalla mappa
    method_count.remove("main");

    method_count.len()
}

pub fn count_impl_blocks(file_path: &str) -> usize {
    let content = fs::read_to_string(file_path).expect("Errore nella lettura del file");
    let re = Regex::new(r"impl\s+\w+\s*\{").unwrap();
    re.find_iter(&content).count()
}

pub fn count_struct_blocks(file_path: &str) -> usize {
    let content = fs::read_to_string(file_path).expect("Errore nella lettura del file");
    let re = Regex::new(r"struct\s+\w+\s*[{;]").unwrap();
    re.find_iter(&content).count()
}

use crate::project_analyzer::class_manager;
use crate::project_analyzer::functions_finder;
use std::{collections::HashMap, path::Path};

pub fn detect_blob(profiling_folder: String) {

    //Ritorno alla cartella precedente
    let path = Path::new(&profiling_folder).parent().map(|p| p.display().to_string()).unwrap();

    let result: HashMap<String, HashMap<String, Vec<String>>> = functions_finder::find_functions_by_folder(true, &path, 1);
        
    if result.is_empty(){
        println!("Nessuna funzione pubblica trovata");
        return;
    }

    // println!("\n📌 Albero del progetto:");
    // println!("{:#?}", result);

    //show_tree_methods(result, &path);

}


pub fn detect_god_class(file_name: String, file_path: &str) {
    //Conta i metodi totali
    let god_class_threshold = 10;

    let method_count = class_manager::count_methods(file_path);
    let impl_count = class_manager::count_impl_blocks(file_path);
    let struct_count = class_manager::count_struct_blocks(file_path);

    println!("  - Numero di metodi del file {}: {}", file_name, method_count);
    println!("  - Numero di implementazioni del file {}: {}", file_name, impl_count);
    println!("  - Numero di struct del file {}: {}", file_name, struct_count);

    if impl_count > 1 && struct_count > 1 && method_count > god_class_threshold {
        println!("");
        println!("⚠️ La classe {} è un God Class contentente {} metodi, {} implementazioni e {} struct ⚠️", file_name, method_count, impl_count, struct_count);
        println!("");
    }

}
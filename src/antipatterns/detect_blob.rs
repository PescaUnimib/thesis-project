use crate::project_analyzer::class_manager::show_tree_methods;
use crate::project_analyzer::functions_finder;
use std::collections::HashMap;

pub fn detect_blob(path: String) {

    let result: HashMap<String, HashMap<String, Vec<String>>> = functions_finder::find_functions_by_folder(true, &path);
        
    if result.is_empty(){
        println!("Nessuna funzione pubblica trovata");
        return;
    }

    show_tree_methods(result, &path);

    

}


pub fn detect_god_class(functions_map: Vec<(String, i32)>, impl_count: usize, struct_count: usize) {
    //Conta i metodi totali
    let total_methods = functions_map.len();
    let god_class_threshold = 10;

    if impl_count > 1 && struct_count > 1 && total_methods > god_class_threshold {
        println!("La classe è un God Class, si consiglia di suddividerla in classi più piccole");
    } else {
        println!("La classe non è un God Class");
    }

}
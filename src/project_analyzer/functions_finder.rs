use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use syn::{Item, ItemImpl, ImplItem, ItemFn};

use crate::project_analyzer::class_manager::*;
use crate::antipatterns::*;

/// Stampa la mappa dei file di progetto
pub fn find_functions_by_folder(is_public: bool, path: &str, option: i32) -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut folder_map: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let base_path = Path::new(path);

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy().ends_with(".rs"))
    {
        let file_path = entry.path();

        if let Ok(relative_path) = file_path.strip_prefix(base_path) {
            let mut folders = Vec::new();
            for folder in relative_path.parent().unwrap_or_else(|| Path::new("")).iter() {
                if folder != "src" {
                    folders.push(folder.to_string_lossy().to_string());
                }
            }

            let file_name = file_path.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            if file_name != "mod.rs" && file_name != "lib.rs" {

                println!("Analisi del file {}", file_name);

                match option {
                    //Detection del Blob
                    1 => {
                        detect_god_class::detect_god_class(file_name.clone(), file_path.to_str().unwrap());
                    },

                    _ => {
                        println!("Opzione non riconosciuta: {}", option);
                    },
                }
    
                let content = fs::read_to_string(file_path).unwrap_or_else(|_| String::new());
    
                if let Ok(parsed) = syn::parse_file(&content) {
                    let mut functions = Vec::new();
    
                    for item in parsed.items {
                        match item {
                            // Funzioni globali
                            Item::Fn(ItemFn { vis, sig, .. }) => {
                                let is_pub = matches!(vis, syn::Visibility::Public(_));
                                if is_public && is_pub || !is_public {
                                    functions.push(sig.ident.to_string());
                                }
                            }
                            // Metodi dentro impl
                            Item::Impl(ItemImpl { items, .. }) => {
                                for impl_item in items {
                                    if let ImplItem::Fn(method) = impl_item {
                                        functions.push(method.sig.ident.to_string());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
    
                    if !functions.is_empty() {
                        let full_path = folders.join("/");
                        folder_map.entry(full_path.clone())
                            .or_insert_with(HashMap::new)
                            .entry(file_name)
                            .or_insert_with(Vec::new)
                            .extend(functions);
                    }
                }
            }
        }
    }
    folder_map
}

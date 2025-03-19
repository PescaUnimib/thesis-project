use inquire::Select;
use inquire::Text;
use project_analyzer::benchmark_manager::*;
use project_analyzer::class_manager::show_tree_methods;
use antipatterns::*;
use project_analyzer::*;

use std::collections::HashMap;
use std::path;
use std::path::Path;
use std::process::{Command, exit};
use std::result;

mod project_analyzer;
mod antipatterns;

fn main() {
    // println!("Benvenuto su RPAD, scegli una delle opzioni");

    // let options = vec!["Ispeziona codice", "Informazioni sul progetto", "Esci"];

    // // Mostra il menu e ottieni la selezione
    // let selection = Select::new("Scegli una delle opzioni:", options.clone())
    //     .prompt()  // Mostra il menu
    //     .unwrap(); // Gestisce l'errore

    // println!("Hai scelto: {}", selection);

    // match selection {
    //     "Ispeziona codice" => source_finder(),
    //     "Informazioni sul progetto" => println!("Mostra info"),
    //     "Esci" => {
    //         println!("Uscendo...");
    //         return;
    //     },
    //     _ => println!("Opzione non valida"),
    // }

    //source_finder();

    //TODO da rimuovere e sostituire con source_finder()
    run_program("C:\\Projects\\Rust\\example\\example_project".to_string());

    fn source_finder() {
        loop {
            let path = Text::new("Inserisci il percorso del tuo progetto Rust: ")
                .prompt()
                .unwrap();
    
            if path.is_empty() {
                println!("Errore: Il percorso non può essere vuoto.");
                continue;
            }
    
            let path_obj = Path::new(&path);
            if !path_obj.exists() {
                println!("Errore: Il percorso inserito non esiste.");
                continue;
            }
    
            if !path_obj.is_dir() {
                println!("Errore: Il percorso non è una directory.");
                continue;
            }
    
            println!("Hai inserito un percorso valido: {}", path);
            run_program(path);

            break;
        }
    }

    fn run_program(path: String){

        //   C:\Projects\Rust\example\example_project

        let result: HashMap<String, HashMap<String, Vec<String>>> = functions_finder::find_functions_by_folder(true, &path);
        
        if result.is_empty(){
            println!("Nessuna funzione pubblica trovata per il benchmark");
            return;
        }

        show_tree_methods(result, &path);




        //benchmark_manager::manage_benchmark(&path, result);

        let mut function_execution_time_map: HashMap<String, f64> = HashMap::new();
        //function_execution_time_map = data_manager::get_function_execution_time(path);
        
    }



}

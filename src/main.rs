use inquire::Select;
use inquire::Text;
use thesis_project::profiler_reader::summary;

use std::error::Error;
use std::path;
use std::path::Path;
use std::process::{Command, exit};
use std::result;
use thesis_project::profiler_reader;

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

    //TODO da rimuovere e sostituire con source_finder() in riga 39
    //let _ = run_program("C:\\Projects\\Rust\\example\\example_project".to_string());

    choose_antipattern("C:\\Projects\\Rust\\example\\example_project".to_string());

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
            // choose_antipattern(path);
            //let _ = run_program(path);

            break;
        }
    }

    fn run_program(path: String) -> Result<(), Box<dyn Error>>{

        // C:\Projects\Rust\example\example_project

        // C:\Projects\Rust\example\example_project\sample (matrix)\r000hs\report.csv
        let filename = "C:\\Projects\\Rust\\example\\example_project\\sample (matrix)\\r000hs\\report.csv";
        // let execution_times = summary::hotspot_reader::extract_execution_times(filename)?;

        // for (function, time) in &execution_times {
        //     println!("{} -> {:.6}s", function, time);
        // }

        let another_filename = "C:\\Projects\\Rust\\example\\example_project\\sample (matrix)\\r004macc\\report.csv";

        summary::memory_access_reader::read_memory_access_summary(another_filename)?;

        Ok(())

    }

    fn choose_antipattern(path: String) {
        let options = vec!["Blob", "Wrong Cache Strategy", "Esci"];
    
        let selection = Select::new("Scegli una delle opzioni:", options.clone())
            .prompt()  // Mostra il menu
            .unwrap(); // Gestisce l'errore
    
        println!("Hai scelto: {}", selection);
    
        if let Some(index) = options.iter().position(|&s| s == selection) {
            match index {
                0 => antipatterns::detect_blob::detect_blob(path),
                1 => antipatterns::detect_wcs::detect_wcs(path),
                2 => {
                    println!("Uscendo...");
                    return;
                },
                _ => println!("Opzione non valida"),
            }
        } else {
            println!("Opzione non valida");
        }
    }



}

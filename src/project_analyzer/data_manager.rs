use scraper::{Html, Selector};
use std::fs;
use std::path::Path;

#[allow(dead_code)]
pub fn get_function_execution_time(path: String) -> std::collections::HashMap<String, f64> {
    let mut function_execution_time_map: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    let path = format!("{}\\target\\criterion", path);

    let mut execution_time = 0.0;
    let mut function_name = String::new();

    println!("Percorso: {}", path);

    // Verifica se il file esiste
    if Path::new(&path).exists() {
        println!("Il file esiste: {}", path);
    } else {
        println!("Il file non esiste.");
    }

    //Per ogni cartella nella directory target/criterion si cerca il file report
    for entry in fs::read_dir(&path).expect("Errore nella lettura della directory") {
        let entry = entry.expect("Errore nella lettura del file");
        let path = entry.path();
        if path.is_dir() && !path.to_str().unwrap().contains("report") {
            println!("Cartella trovata: {}", path.display());
            let file_name = path.join("report\\index.html");
            println!("File report: {}", file_name.display());
            function_name = path.file_name().unwrap().to_str().unwrap().to_string();
            println!("Nome funzione: {}", function_name);

            // Prova a leggere il file HTML
            let html = fs::read_to_string(file_name).expect("Errore nel leggere il file HTML");
            
            // Parsing dell'HTML
            let document = Html::parse_document(&html);
            
            // Selettore per trovare la tabella dei valori (puoi specificare il selettore esatto per il valore Slope)
            let selector = Selector::parse("table tbody tr:nth-child(1) td:nth-child(3)").unwrap();
            
            // Trova e stampa il valore
            if let Some(element) = document.select(&selector).next() {
                let slope_value = element.text().collect::<Vec<_>>().join("").trim().to_string();
                let cleaned_value = slope_value.split_whitespace().next().unwrap(); // Rimuove "ns" o altre unità
                execution_time = cleaned_value.parse::<f64>().expect("Errore nella conversione a float");                
            } else {
                println!("Slope non trovato");
            }

            function_execution_time_map.insert(function_name.clone(), execution_time);
        }
    }

    //Stampa la mappa
    for (key, value) in &function_execution_time_map {
        println!("Mappa dei tempi di esecuzione {}: {}", key, value);
    }

    function_execution_time_map

}

pub fn count_function_calls() -> std::collections::HashMap<String, f64> {
    let mut function_calls_map: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    function_calls_map
}


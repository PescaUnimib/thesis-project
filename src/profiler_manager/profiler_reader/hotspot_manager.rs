use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use csv::ReaderBuilder;
use walkdir::WalkDir;

pub fn check_hotspot_folder(profiling_folder: String) -> Option<String> {
    for entry in WalkDir::new(&profiling_folder).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let file_path = entry.path();

        // Verifica se l'entry è una cartella
        if file_path.is_dir() {
            if let Some(name) = file_path.file_name() {
                if name.to_string_lossy().ends_with("hs") {
                    println!("Cartella Hotspot trovata: {}", file_path.display());
                    return Some(file_path.display().to_string());
                }
            }
        }
    }

    println!("Cartella Hotspot non trovata");
    None
}

pub fn read_hotspot_summary (profiling_folder: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(profiling_folder)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .flexible(true)
        .has_headers(false)
        .from_reader(file);

    let mut records = Vec::new();
    let mut col_widths = Vec::new();

    // 1️⃣ Leggi tutte le righe e trova la larghezza massima di ogni colonna
    for result in rdr.records() {
        let record = result?;
        if col_widths.len() < record.len() {
            col_widths.resize(record.len(), 0);
        }
        for (i, field) in record.iter().enumerate() {
            col_widths[i] = col_widths[i].max(field.len());
        }
        records.push(record);
    }

    // 2️⃣ Stampa i dati allineati
    for record in records {
        for (i, field) in record.iter().enumerate() {
            print!("{:<width$} ", field, width = col_widths[i]); // Allineamento a sinistra
        }
        println!();
    }

    Ok(())
}

//Inserimento tempi di esecuzione dei metodi

pub fn extract_execution_times(hotspot_folder: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let formatted_hotspot_folder = format!("{}", hotspot_folder);

    let output = Command::new("vtune")
        .args(&["-report", "hotspots", "-r", &formatted_hotspot_folder, "--column=Function,Module,CPU Time"])
        .output()?; // Propaga gli errori se fallisce

    // Stampa l'output per il debug
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr)); // Stampa stderr

    if !output.status.success() {
        return Err(format!("Vtune non ha avuto successo, codice di uscita: {}", output.status).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut execution_times = HashMap::new();
    let mut is_first_line = true; // Flag per saltare la riga dei titoli

    for line in stdout.lines() {
        // Salta la riga dei titoli
        if is_first_line {
            is_first_line = false;
            continue;
        }

        let columns: Vec<&str> = line.split_whitespace().collect();

        // Aggiungi un controllo per il numero minimo di colonne
        if columns.len() < 6 {
            println!("Riga ignorata (non abbastanza colonne): {}", line);  // Debug per vedere quali righe vengono ignorate
            continue;
        }

        let function_name = columns[6];  // Funzione
        let module_name = columns[5];    // Nome del modulo
        let cpu_time_str = columns[1];  // Tempo di esecuzione 

        // Rimuovi "s" dal tempo di esecuzione e converti in f64
        let cpu_time: f64 = cpu_time_str.trim_end_matches('s').parse().unwrap_or(0.0);

        if module_name.ends_with(".exe") {
            println!("Inserimento di {} -> {}", function_name, cpu_time);  // Debug per vedere cosa viene inserito
            execution_times.insert(function_name.to_string(), cpu_time);
        }
    }

    if execution_times.is_empty() {
        println!("Nessun modulo .exe trovato o nessun dato valido.");  // Aggiungi questa linea per debug
    }

    Ok(execution_times)
}
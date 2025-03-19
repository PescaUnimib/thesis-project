use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::ReaderBuilder;

pub fn read_hotspot_summary<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
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
pub fn extract_execution_times<P: AsRef<Path>>(filename: P) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .flexible(true)
        .has_headers(false)
        .from_reader(file);

    let mut execution_times = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        if record.len() < 4 {
            continue;
        }

        let function_name = record[1].trim(); // Metric Name
        let module_name = record[2].trim(); // Metric Value
        let cpu_time: f64 = record[3].trim().parse().unwrap_or(0.0); // CPU Time

        if module_name.ends_with(".exe") {
            execution_times.insert(function_name.to_string(), cpu_time);
        }
    }

    Ok(execution_times)
}

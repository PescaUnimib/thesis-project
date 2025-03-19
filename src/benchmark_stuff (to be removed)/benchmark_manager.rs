// use std::fs;
// use std::path::Path;
// use std::process::Command;
// use std::collections::HashMap;
// use std::fs::{File, OpenOptions};
// use std::io::{self, Read, Write};

// //Probabilmente inutile
// pub fn manage_benchmark(path: &str, result: HashMap<String, HashMap<String, Vec<String>>>) {
//     let main_folder = extract_main_folder_name(&path).unwrap();
//     let mut vec: Vec<String> = Vec::new();
//     let mut benchmark_map: HashMap<String, Vec<String>> = HashMap::new();
//     let mut i = 1;

//     for (folder, files) in result.iter() {
//         vec.push(main_folder.clone());
//         vec.push(folder.to_string());
//         for (file, functions) in files.iter() {
//             let mut functions_in_folders: Vec<String> = Vec::new();
//             for function in functions {
//                 functions_in_folders.push(function.to_string());
//             }
//             benchmark_map.insert(file.to_string(), functions_in_folders.clone());
//             vec = check_for_subdirectories(&vec);
//             vec.push("*".to_string());
//             create_benchmarks(&path, &vec,i, benchmark_map.clone());
//             vec.pop();

//             println!("Mappa dei benchmark: {:?}", benchmark_map);
//             println!("Percorso: {:?}", vec);
//         }
        
//         println!("------------------------------------------");

//         vec.clear();
//         benchmark_map.clear();
//         i += 1;
//     }

//     run_benchmark(&path);
// }

// fn extract_main_folder_name(path: &str) -> Option<String> {
//     let path = Path::new(path);
//     path.file_name()
//         .and_then(|name| name.to_str())
//         .map(|s| s.to_string())
// }

// fn check_for_subdirectories(vec: &Vec<String>) -> Vec<String> {
//     let result: Vec<String> = vec
//     .into_iter()
//     .flat_map(|s| s.split('/').map(|s| s.to_string()))
//     .collect();
//     result
// }

// pub fn create_benchmarks(path: &str, vec: &[String], index: i32, benchmark_map: HashMap<String, Vec<String>>) {
//     let benches_path = Path::new(path).join("benches");
//     let benchmark_file = benches_path.join(format!("benchmark{}.rs", index));

//     modify_toml(path, index).expect("Errore nella modifica del file Cargo.toml");

//     // Crea la cartella benches se non esiste
//     if !benches_path.exists() {
//         fs::create_dir(&benches_path).expect("Errore nella creazione della cartella benches");
//     }

//     // importazione delle funzioni
//     let import_path = vec.join("::");
//     let mut benchmark_code = format!(
//         r#"
//             use criterion::{{
//                 black_box, criterion_group, criterion_main, Criterion
//             }};
//             use {};
//         "#,
//         import_path
//     );

//     // creazione dei benchmark
//     for benchmark in &benchmark_map {
//         let file_name = benchmark.0.replace(".rs", "");
//         for function in benchmark.1 {
//             benchmark_code.push_str(&format!(
//                 r#"
//                     fn bench_{}(c: &mut Criterion) {{
//                         c.bench_function("{}", |b| b.iter(|| black_box({}::{}())));
//                     }}
//                 "#, function, function, file_name, function
//             ));
//         }
//     }
    
//     benchmark_code.push_str("\ncriterion_group!(benches, ");
//     benchmark_code.push_str(benchmark_map.values().map(|f| f.iter().map(|f| format!("bench_{}", f)).collect::<Vec<_>>().join(", ")).collect::<Vec<_>>().join(", ").as_str());
//     benchmark_code.push_str(");\ncriterion_main!(benches);");

//     fs::write(&benchmark_file, benchmark_code).expect("Errore nella scrittura del benchmark");
// }

// #[allow(dead_code)]
// pub fn run_benchmark(path: &str) {
//     let status = Command::new("cargo")
//         .arg("bench")
//         .current_dir(path)
//         .status()
//         .expect("Errore nell'esecuzione del benchmark");

//     if status.success() {
//         println!("Benchmark completato con successo!");
//     } else {
//         println!("Errore durante il benchmark.");
//     }
// }

// pub fn modify_toml(path: &str, index: i32) -> io::Result<()> {

//     let file_path = Path::new(path).join("Cargo.toml");

//     let bench_name = format!("benchmark{}", index);

//     // Esempio di nomi di benchmark dinamici (puoi modificarli come desideri)
//     //let bench_names = vec!["benchmark1".to_string(), "benchmark2".to_string(), "benchmark3".to_string()];

//     // Leggi il file esistente
//     let mut file = File::open(file_path.clone())?;
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;

//     // Verifica se un bench esiste già nel contenuto
//     let mut toml_content = content.clone();

//     // Se non esiste, aggiungilo
//     if !toml_content.contains(&format!("name = \"{}\"", bench_name)) {
//         let bench_entry = format!("\n[[bench]]\nname = \"{}\"\nharness = false\n", bench_name);
//         toml_content.push_str(&bench_entry);
//     }

//     // for bench_name in bench_names {
//     //     // Controlla se il benchmark esiste già
//     //     if !toml_content.contains(&format!("name = \"{}\"", bench_name)) {
//     //         // Se il bench non esiste, aggiungilo
//     //         let bench_entry = format!("\n[[bench]]\nname = \"{}\"\nharness = false\n", bench_name);
//     //         toml_content.push_str(&bench_entry); // Aggiungi il nuovo benchmark
//     //     }
//     // }

//     // Scrivi il file modificato
//     let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
//     file.write_all(toml_content.as_bytes())?;

//     Ok(())
// }

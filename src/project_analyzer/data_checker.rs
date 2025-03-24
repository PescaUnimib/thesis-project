///This class check if profiling data existst in the given path

use walkdir::WalkDir;

pub fn check_data(path: String) -> Option<String> {
    for entry in WalkDir::new(&path) {
        let entry = entry.unwrap();
        let file_path = entry.path();

        // Controlla se il file ha estensione .vtuneproj
        if let Some(ext) = file_path.extension() {
            if ext == "vtuneproj" {
                println!("Cartella di profiling trovata: {}", file_path.parent().map(|p| p.display().to_string()).unwrap());
                return file_path.parent().map(|p| p.display().to_string());
            }
        }
    }
    println!("Nessuna cartella di profiling trovata.");
    None
}

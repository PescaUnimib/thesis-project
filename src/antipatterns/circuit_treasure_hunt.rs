#![allow(warnings)]
use std::collections::HashMap;
use sysinfo::System;
use crate::project_analyzer::class_manager;

/// Simula il recupero dell'uso medio della CPU.
fn get_avg_cpu_usage() -> f32 {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    let cpu_usage: f32 = sys.global_cpu_info().cpu_usage();
    cpu_usage
}

/// Simula il recupero della mappa dei conteggi dei metodi.
fn get_method_count_map(option: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    map.insert("methodA".to_string(), 150);
    map.insert("methodB".to_string(), 300);
    map.insert("methodC".to_string(), 450);
    map
}

/// Calcola la media dei conteggi dei metodi.
fn get_avg_method_count(method_count_map: &HashMap<String, usize>) -> f64 {
    let sum: usize = method_count_map.values().sum();
    sum as f64 / method_count_map.len() as f64
}

/// Simula il recupero della media del tempo di esecuzione dei metodi hotspot.
fn get_avg_time(hotspot_methods: &HashMap<String, f64>) -> f64 {
    let sum: f64 = hotspot_methods.values().sum();
    sum / hotspot_methods.len() as f64
}

/// Simula la segnalazione di un potenziale Circuitous Treasure Hunt.
fn report_cth(avg_cpu_usage: f32, method_name: &str, hs_method_count: usize) {
    println!(
        "⚠️ Circuitous Treasure Hunt detected! CPU Usage: {:.2}% | Method: {} | Count: {}",
        avg_cpu_usage, method_name, hs_method_count
    );
}

/// Funzione principale per rilevare il Circuitous Treasure Hunt.
fn detect_cth(count_offset: f64, cpu_th: f32, option: &str) {
    let avg_cpu_usage = get_avg_cpu_usage();
    
    if avg_cpu_usage > cpu_th {
        let method_count_map = get_method_count_map(option);
        let avg_count = get_avg_method_count(&method_count_map);

        let hotspot_methods: HashMap<String, f64> = HashMap::from([
            ("methodA".to_string(), 120.0),
            ("methodB".to_string(), 320.0),
            ("methodC".to_string(), 500.0),
        ]);

        let avg_time_hotspot = get_avg_time(&hotspot_methods);

        for (method, &time) in &hotspot_methods {
            if time > avg_time_hotspot {
                if let Some(&hs_method_count) = method_count_map.get(method) {
                    let count_th = avg_count * (1.0 + count_offset);
                    if (hs_method_count as f64) > count_th {
                        report_cth(avg_cpu_usage, method, hs_method_count);
                    }
                }
            }
        }
    }
}
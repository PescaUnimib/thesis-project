
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
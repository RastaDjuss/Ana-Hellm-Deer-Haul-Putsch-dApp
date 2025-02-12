fn main() {
    // Transmet une configuration personnalisée au compilateur si elle est nécessaire
    println!("cargo:rustc-check-cfg=cfg(foo, values(\"bar\"))");

    // Exemple de condition personnalisée
    let foo_bar_condition = true; // Si la logique détermine que ces options sont nécessaires.

    // Applique une configuration si la condition est remplie
    if foo_bar_condition {
        println!("cargo:rustc-cfg=foo=\"bar\""); // Passe une directive au compilateur.
    }
}
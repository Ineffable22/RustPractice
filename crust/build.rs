fn main() {
    // Especifica la ubicación de la biblioteca libsum.a
    println!("cargo:rustc-link-search=native=./lib");
    println!("cargo:rustc-link-lib=static=sum");
}

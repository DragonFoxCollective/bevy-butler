fn main() {
    #[cfg(not(feature="inventory"))]
    println!("cargo:rustc-link-arg=-znostart-stop-gc");
}
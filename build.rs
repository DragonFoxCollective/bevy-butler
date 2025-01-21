fn main() {
    #[cfg(not(feature = "inventory"))]
    println!("cargo:rustc-link-arg=-znostart-stop-gc");

    if std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|target| target == "wasm32") {
        println!("cargo:warning=WEBASSEMBLY SUPPORT IS EXPERIMENTAL AND WILL PROBABLY NOT WORK!");
        println!("cargo:warning=See: https://github.com/TGRCdev/bevy-butler/issues/3")
    }
}

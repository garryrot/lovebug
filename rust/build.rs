
fn main() {
    let bridges = vec!["src/lib.rs","src/logging.rs"];
    cxx_build::bridges(bridges)
        .include("src")
        .include("lbug/src")
        .compile("lovebug");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/logging.rs");
}

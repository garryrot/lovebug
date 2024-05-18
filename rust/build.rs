
fn main() {
    let bridges = vec!["src/lib.rs","src/logging.rs", "src/events.rs"];
    let _bb = cxx_build::bridges(bridges);
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/logging.rs");
    println!("cargo:rerun-if-changed=src/events.rs");
}

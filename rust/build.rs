
fn main() {
    let bridges = vec!["src/lib.rs", "src/logging.rs",  "src/bridge.rs", "src/events.rs", "src/bones.rs", "src/mcm.rs"];
    let _bb = cxx_build::bridges(bridges);
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/logging.rs");
    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rerun-if-changed=src/events.rs");
    println!("cargo:rerun-if-changed=src/bones.rs");
    println!("cargo:rerun-if-changed=src/mcm.rs");
}

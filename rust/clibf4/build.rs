// build.rs
fn main() {
    let bridges = vec![
        "src/bridge.rs"
    ];
    
    let _bb = cxx_build::bridges(bridges);
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rerun-if-changed=src/wrapper.rs");
}
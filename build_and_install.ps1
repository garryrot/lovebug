# cargo build is triggered by cmake, but will only recognize source
# changes in explicitly listed files, hence always trigger it
Set-Location ./rust
cargo build --release
Set-Location ..

cmake --build --preset build --config Release

./build_scripts.ps1
./install_mod.ps1

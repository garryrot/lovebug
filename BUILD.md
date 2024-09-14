# Building Native Plugin

This build generates `Lovebug.dll`, a F4SE plugin based on CommonLibF4 NG.

There should really be no need for you to do this, unless you:

- Want to port the native library to a skyrim version that I currently don't support
- Want to fork this project and change the native library

## Build Requirements

 1. [Rust](https://www.rust-lang.org/tools/install) - executables like `cargo` should be present in your PATH
 2. [Visual Studio 2022](https://visualstudio.microsoft.com/de/) with a C++ compiler
 3. [CMake](https://cmake.org/download/) - make sure that its added to your PATH environment variable
 4. [VCPKG](https://github.com/microsoft/vcpkg) 
    - Set environment variable VCPKG_ROOT to the vcpkg installation folder
    - Version must be 2023-02-16 or later, otherwise youll get `xbox` error

## Step-By-Step

1. Test VCPKG_ROOT is set in your build terminal. This should return the path:

```sh
echo %VCPKG_ROOT%
```

2. Init submodules

```sh
git submodule update --init --recursive
```

3. Build the project

```sh
cmake --preset build
cmake --build --preset build --config Release
```

# Building Papyrus Scripts 

*to be done*

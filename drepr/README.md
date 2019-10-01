# Installation

1. Install Rust
2. Install Anaconda 3
3. Build the program: `cargo build`

### Possible Errors

If you want to learn how Rust handle cross compiling, see [here](https://github.com/japaric/rust-cross).

1.. Incompatible GCC version.

```.env
lto1: fatal error: bytecode stream generated with LTO version 6.0 instead of the expected 5.2
```
 
You may encounter the above error if your gcc version is different with gcc that Anaconda uses to build Python (7.3.0). The newer GCC produce code with LTO 6.0, while the `cc` Rust is using produces code with LTO 5.2. To fix this error, install the gcc that Anaconda 3 uses by:

    conda install gxx_linux-64

Then, tell cargo to build with anaconda's cc [linker](https://users.rust-lang.org/t/compiling-rust-package-using-cc-linker-from-a-custom-location/15795) via environment variable `CARGO_TARGET_<triple>_LINKER=<path_to_x86_64-conda_cos6-linux-gnu-cc> cargo build` or via configuration in `.cargo/config`.

2.. Segmentation fault when running the python library or Undefined reference to `_Py_Dealloc` when building test

When building the dylib, the cpython library requires features `extension-module`. This feature creates error when building binary program or running test in linux, so we disable it. You have to enable them manually when building library by telling cargo to build lib and the feature `cpython/extension-module` is enable:

    cargo build --lib --features cpython/extension-module 
 
Using workspace on linux required that you need to be in the package directory (not workspace directory) to run the above command
and get a correct library, otherwise, you may encounter segmentation fault. For example, I need to be in the
engine directory to run the above command  

3.. Error while running doctest

If you are running test with cross-compiler, the doctest is likely to fail due to cargo doesn't pass the correct linker. You can ignore the doctest by running `cargo test --tests`

4.. Fail to run the binary (Fatal Python error: ModuleNotFoundError: No module named: `encodings`)

You need to specify the PYTHONHOME environment variable because the program cannot find the python libraries. 

a. In Window:

    $env:PYTHONHOME="C:\ProgramData\Anaconda3"
    
b. In MacOS, Linux:
    
    export PYTHONHOME=$ANACONDA_HOME
    
5.. Missing <netcdf.h>

The `netcdf` format requires to compile with `libnetcdf`, we need to install the package and update `CPATH` to point to the correct include directory of anaconda

    conda install libnetcdf
    export CPATH=$CPATH:$ANACONDA_HOME/include

6.. Loading library error

On MacOS, cargo produces dynamic linking for libstd (`@rpath\libstd...`)

1. https://github.com/rust-lang/cargo/issues/7226
2. https://stackoverflow.com/questions/55282165/dylib-cannot-load-libstd-when-compiled-in-a-workspace

You can check linking on MacOS using otool 

### Awared Issues

1. It is very slow to build

Rust-cpython is very slow to build. You can disable the python feature if you are in the debug mode using `--features "disable-python readers/disable-python"` flag.
However, it only works if you are in the crate folder, cargo does not support passing `features` flag in the workspace folder yet ([see more](https://github.com/rust-lang/cargo/issues/5015))


# Useful commands

1. Code coverage: `python devel cov`
2. Building python library: `python devel pylib-build`
3. Release python library: `python devel pylib-release`. The python library can be installed by running: `pip install git+https://github.com/binh-vu/temp.git@pyrepr`


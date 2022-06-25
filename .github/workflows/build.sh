#!/bin/bash

set -e

# Description: builds Python's wheels.
#
# Envionment Arguments: (handled by `args.py`)
#   PYTHON_HOME: the path to the Python installation, which will be used to build the wheels for. 
#        Empty if you want to use multiple Pythons by providing PYTHON_HOMES. This has the highest priority. If set, we won't consider PYTHON_HOMES and PYTHON_VERSIONS
#   PYTHON_HOMES: comma-separated directories that either contains Python installations or are Python installations.
#   PYTHON_VERSIONS: versions of Python separated by comma if you want to restricted to specific versions.
#   TARGET: see https://doc.rust-lang.org/nightly/rustc/platform-support.html

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if [ -z "$TARGET" ]
then
    echo "TARGET is not set. https://doc.rust-lang.org/nightly/rustc/platform-support.html"
    exit 1
fi

echo "::group::Install Rust"
if ! command -v cargo &> /dev/null
then
    # install rust and cargo
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH=$HOME/.cargo/bin:$PATH
else
    echo "Rust is already installed."
    rustup show
fi

if [[ ! -d $(rustc --print target-libdir --target "$TARGET" )]]; then
    rustup target add $TARGET
fi
echo "::endgroup::"
echo

echo "::group::Discovering Python"
IFS=',' read -a PYTHON_HOMES <<< $(python $SCRIPT_DIR/pydiscovery.py)
if [ ${#PYTHON_HOMES[@]} -eq 0 ]; then
    echo "No Python found. Did you forget to set any environment variable PYTHON_HOME or PYTHON_HOMES?"
fi
echo "::endgroup::"
echo

for PYTHON_HOME in "${PYTHON_HOMES[@]}"
do
    echo "::group::Building for $PYTHON_HOME"
    
    echo "::group::Install Building Tools"
    "$PYTHON_HOME/bin/pip" install maturin
    echo "::endgroup::"
    echo

    echo "::group::Building Wheels"
    echo "Run: $PYTHON_HOME/bin/maturin build -r -o dist -i $PYTHON_HOME/bin/python --target $TARGET"
    "$PYTHON_HOME/bin/maturin" build -r -o dist -i "$PYTHON_HOME/bin/python" --target $TARGET
    echo "::endgroup::"

    echo "::endgroup::"
    echo
done

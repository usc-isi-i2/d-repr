#!/bin/bash

set -e

# Description: builds Python's wheels.
# The script needs yum or apt
#
# Envionment Arguments: (handled by `args.py`)
#   PYTHON_HOME: the path to the Python installation, which will be used to build the wheels for. 
#        Empty if you want to use multiple Pythons by providing PYTHON_HOMES. This has the highest priority. If set, we won't consider PYTHON_HOMES and PYTHON_VERSIONS
#   PYTHON_HOMES: comma-separated directories that either contains Python installations or are Python installations.
#   PYTHON_VERSIONS: versions of Python separated by comma if you want to restricted to specific versions.
# Arguments:
#   -t <target>: target platform. See https://doc.rust-lang.org/nightly/rustc/platform-support.html

export PATH=$EXTRA_PATH:$PATH

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

while getopts t: flag
do
    case "${flag}" in
        t) target=${OPTARG};;
    esac
done

if [ -z "$target" ]
then
    echo "target is not set (-t <target>). See more: https://doc.rust-lang.org/nightly/rustc/platform-support.html"
    exit 1
fi

echo "::group::Setup build tools"
# ##############################################
# to build rocksdb, we need CLang and LLVM
echo "Install CLang and LLVM"
if ! command -v yum &> /dev/null
then
    # debian
    apt update
    apt install -y clang-11
else
    # centos
    # https://developers.redhat.com/blog/2018/07/07/yum-install-gcc7-clang#
    yum install -y llvm-toolset-7
    source /opt/rh/llvm-toolset-7/enable
fi

# ##############################################
echo "Install Rust"
if ! command -v cargo &> /dev/null
then
    # install rust and cargo
    curl --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
    source $HOME/.cargo/env
else
    echo "Rust is already installed."
    rustup show
fi

if [ ! -d $(rustc --print target-libdir --target "$target" ) ]
then
    rustup target add $target;
fi

echo "::endgroup::"
echo

echo "::group::Discovering Python"
IFS=',' read -a PYTHON_HOMES <<< $(MINIMUM_PYTHON_VERSION=3.8 python $SCRIPT_DIR/pydiscovery.py)
if [ ${#PYTHON_HOMES[@]} -eq 0 ]; then
    echo "No Python found. Did you forget to set any environment variable PYTHON_HOME or PYTHON_HOMES?"
else
    for PYTHON_HOME in "${PYTHON_HOMES[@]}"
    do
        echo "Found $PYTHON_HOME"
    done    
fi
echo "::endgroup::"
echo

for PYTHON_HOME in "${PYTHON_HOMES[@]}"
do
    echo "::group::Building for Python $PYTHON_HOME"

    echo "Run: $PYTHON_HOME/bin/pip install maturin"    
    "$PYTHON_HOME/bin/pip" install maturin

    echo "Run: $PYTHON_HOME/bin/maturin build -r -o dist -i $PYTHON_HOME/bin/python --target $target"
    "$PYTHON_HOME/bin/maturin" build -r -o dist -i "$PYTHON_HOME/bin/python" --target $target

    echo "::endgroup::"
    echo
done

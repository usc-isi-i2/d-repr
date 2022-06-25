#!/bin/bash

set -e

# Description: builds Python's wheels.

if ! command -v cargo &> /dev/null
then
    # install rust and cargo
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi
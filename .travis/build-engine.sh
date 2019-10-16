#!/bin/bash
set -e

export ENGINE_FILE=$(python ${TRAVIS_BUILD_DIR}/pydrepr/devel info --pre_built_engine_location)

# Build the engine
if [ -f "$ENGINE_FILE" ]; then
    echo ">>> Pre-built engine file exists. Skip building process"
    export HAS_NEW_ENGINE="false"
else
    echo ">>> Pre-built engine file does not exist. Start building it..."
    echo "List and remove previous pre-built files..."
    ls $(python ${TRAVIS_BUILD_DIR}/pydrepr/devel info --pre_built_engine_glob)
    rm $(python ${TRAVIS_BUILD_DIR}/pydrepr/devel info --pre_built_engine_glob)
    echo ">>> Testing..."
    echo ">>> Building..."
    python ${TRAVIS_BUILD_DIR}/pydrepr devel pylib-build -m release
    python ${TRAVIS_BUILD_DIR}/pydrepr devel pylib-release -m release
    export HAS_NEW_ENGINE="true"
fi
#!/bin/bash
set -e

echo "Publish pypi package: tag=$TRAVIS_TAG os=$TRAVIS_OS_NAME"

if [[ ! -z "${TRAVIS_TAG}" && $TRAVIS_OS_NAME = "linux" ]]; then
    pip install twine
    cd ${TRAVIS_BUILD_DIR}/pydrepr
    rm -rf ./dist || echo "No previous build"
    python setup.py sdist
    twine upload -u __token__ -p $PYPI_PWD --skip-existing dist/*
else
    echo "Skip publishing pypi package.."
fi

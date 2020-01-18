#!/bin/bash
set -e

if [ "${TRAVIS_OS_NAME}" = "linux" ]  ; then export MINICONDA_HOME=${TRAVIS_HOME}/miniconda3-${MINICONDA_VERSION}; fi
if [ "${TRAVIS_OS_NAME}" = "osx" ]    ; then export MINICONDA_HOME=${TRAVIS_HOME}/miniconda3-${MINICONDA_VERSION}; fi
if [ "${TRAVIS_OS_NAME}" = "windows" ]; then export MINICONDA_HOME=/c/tools/miniconda3                           ; fi

# solution for three platforms
# the home dir is going to be redundant for linux + osx, and the bin dir is going to be redundant for windows
export PATH=${MINICONDA_HOME}:${MINICONDA_HOME}/bin:$PATH
echo "Install dependencies for ${TRAVIS_OS_NAME}"

##########################
# INSTALL MINICONDA
if [ -f "${MINICONDA_HOME}/travis-${MINICONDA_VERSION}-20191015.txt" ]; then
    echo "Detect that miniconda3 has been installed before. Skip the installation"
else
    echo "${MINICONDA_HOME} not found. Install it!"
    rm -rf ${MINICONDA_HOME}

    if [ "${TRAVIS_OS_NAME}" = "linux" ]; then
        wget https://repo.anaconda.com/miniconda/Miniconda3-${MINICONDA_VERSION}-Linux-x86_64.sh -O /tmp/Miniconda3-${MINICONDA_VERSION}-Linux-x86_64.sh
        mkdir ${TRAVIS_HOME}/.conda
        sh /tmp/Miniconda3-${MINICONDA_VERSION}-Linux-x86_64.sh -b -p ${MINICONDA_HOME}
        conda install -y gxx_linux-64
    fi

    if [ "${TRAVIS_OS_NAME}" = "osx" ]; then
        wget https://repo.anaconda.com/miniconda/Miniconda3-${MINICONDA_VERSION}-MacOSX-x86_64.sh -O /tmp/Miniconda3-${MINICONDA_VERSION}-MacOSX-x86_64.sh
        mkdir ${TRAVIS_HOME}/.conda
        sh /tmp/Miniconda3-${MINICONDA_VERSION}-MacOSX-x86_64.sh -b -p ${MINICONDA_HOME}
    fi

    if [ "${TRAVIS_OS_NAME}" = "windows" ]; then
        choco install miniconda3 --version ${MINICONDA_VERSION} -y
    fi

    echo ${MINICONDA_VERSION} > ${MINICONDA_HOME}/travis-${MINICONDA_VERSION}-20191015.txt
fi

##########################
# INSTALL RUST for Travis Windows
if [ "${TRAVIS_OS_NAME}" = "windows" ]; then
    echo "Install x86_64-pc-windows-msvc target on windows"
    rustup target add x86_64-pc-windows-msvc
fi
#!/usr/bin/env bash

cp ../../Cargo* .
docker build -t isi/drepr:devel .
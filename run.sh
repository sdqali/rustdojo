#!/usr/bin/env bash

rustc "src/$1.rs" -o "target/bin/$1" && "target/bin/$1"

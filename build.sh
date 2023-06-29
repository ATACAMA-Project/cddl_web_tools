#!/bin/bash

set -e

# update code
git reset prod --hard
git pull

# update submodules
git submodule update

# update dependencies
cargo update

# build
cargo build --release
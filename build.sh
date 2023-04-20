#!/bin/bash

cargo build --release

sudo setcap 'cap_net_bind_service=+ep' target/release/cddl_web_tools

# ./target/release/cddl_web_tools
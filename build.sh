#!/usr/bin/env bash

cargo build --release
mkdir oh_hai
cp target/release/oh-hai oh_hai/
cp install oh_hai/
cp uninstall oh_hai/
cp -r shell/ oh_hai/
cargo build --release --bin config
cp target/release/config oh_hai/

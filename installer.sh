#!/bin/bash

cargo build --release -p bin_app --bin bin_app
cp target/release/bin_app ~/.local/bin/mkt

mkt add -p "default-templates/txml.xml" -r
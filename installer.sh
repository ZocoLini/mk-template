#!/bin/bash

cargo build --release -p bin_app --bin bin_app
cp target/release/bin_app ~/.local/bin/mkt

mkt add -p tests/resources/template_example_0.xml -n txml
#!/bin/bash

cargo build --release

YEAR=2025

./target/release/pcap scrapper -o data/ --year ${YEAR} -A

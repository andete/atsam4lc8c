#!/bin/bash

svd2rust -i doc/ATSAM4LC8C.svd
mv src src.`date -u +%s`
form -i lib.rs -o src/ && rm lib.rs
cargo fmt

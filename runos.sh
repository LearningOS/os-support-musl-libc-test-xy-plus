#!/bin/sh

set -e  # stop on error
cd os-ref
make fmt
cd os
make build
cp ./target/riscv64gc-unknown-none-elf/release/os.bin ../../os/
cp ../user/target/riscv64gc-unknown-none-elf/release/fs.img ../../os/
cd ../../os
# ./compress.sh
make run
# timeout 1800 make run > ../output

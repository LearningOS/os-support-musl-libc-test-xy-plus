#!/bin/sh

cd os-ref/os
make build
cp ./target/riscv64gc-unknown-none-elf/release/os.bin ../../os/
cd ../../os
./compress.sh
make run
# timeout 1800 make run > ../output

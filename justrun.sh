#!/bin/sh

cp os-ref/user/target/riscv64gc-unknown-none-elf/release/fs.img os/
cd os
./run.sh
# timeout 1800 make run > ../output

#!/bin/sh

cd os-ref/easy-fs-fuse && cargo run --release -- -s ../user/src/bin/ -t ../user/target/riscv64gc-unknown-none-elf/release/
cd ../..
rm os/fs.img
cp os-ref/user/target/riscv64gc-unknown-none-elf/release/fs.img os/
cd os
./run.sh
# timeout 1800 make run > ../output

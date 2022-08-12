qemu-system-riscv64 \
            -nographic \
            -smp 1 \
            -m 8G \
            -machine virt \
            -kernel ./os.bin \
            -drive file=fs.img,if=none,format=raw,id=x0 \
            -device virtio-blk-device,drive=x0 \
            -bios default

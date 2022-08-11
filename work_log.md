# 工作记录

- 初始化屏幕、键盘、鼠标外设失败

原因是运行测例的 makefile 里，qemu 没有添加外设。在 main 里把外设的初始化删了就好了。

- 初始化 blk 外设失败

因为 virt_blk.rs 里 VIRTIO0 的地址错了。需要创建 fs.img 在运行 qemu 的时候加上 `-drive file=fs.img,if=none,format=raw,id=x0` 和 `-device virtio-blk-device,drive=x0` 。

- easyfs 初始化失败

magic 不对，原因是需要加载通过 user 里生成的 fs.img ，不能直接自己构造空文件。

- 编译 test

修改 prefix 为 `riscv64-linux-musl-`

查找 libc.so

```
> riscv64-linux-musl-gcc --print-file-name=libc.so
/usr/local/riscv64-linux-musl-cross/riscv64-linux-musl/lib/libc.so
```

修改 MUSL_LIB 为 `/usr/local/riscv64-linux-musl-cross/riscv64-linux-musl/lib`

- 无法编译 libc test

脚本有问题，已修复且提交 pr：https://github.com/oscomp/testsuits-for-oskernel/pull/12

- 运行测例报错 [kernel] Segmentation Fault, SIGSEGV=11

暂未解决，可能原因是在 user shell 运行用户程序的时候，没有正确传入参数。

做过的检查：只在 entry.c 里进行 mystrcmp 操作也会报错退出。

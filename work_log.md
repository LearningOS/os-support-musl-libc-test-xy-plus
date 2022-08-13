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

- 偶现无法正常运行 os

暂时通过把 smp 改为 1 解决。

- 编译时间太长

删掉无用的用户程序，保留 initproc 和 usershell ，可以省去编译用户程序的时间。

- 无法编译 libc test

脚本有问题，已修复且提交 pr：https://github.com/oscomp/testsuits-for-oskernel/pull/12

PS：发现 pr 的脚本有一些多余的改动，不过不是很影响，就没提新的 pr ，以本仓库的 fix.py 为准。

- 运行测例报错 `[kernel] Segmentation Fault, SIGSEGV=11`

可能原因：在 user shell 运行用户程序的时候，没有正确传入参数。（应该不是，因为有别的用户程序能正确传入参数）

只在 entry.c 里进行 mystrcmp 操作也会报错退出。

使用仅 return 0 的 enrty.c 放入，仍然报错。

可能原因：没有设置好 \_start 和 entry 等。

参考：https://blog.csdn.net/m0_55708805/article/details/117827482，可以使用ld链接，默认是不会链接运行时库的，需要使用-e参数来 指定程序入口，假如你把程序入口指定为 main，如果你 main 函数没有调用系统调用 exit 的话，那么这个程序执行就会发生 Segementation Fault。这可能是出错的原因。

c riscv64 编译用户进程：https://nankai.gitbook.io/ucore-os-on-risc-v64/lab5/yong-hu-cheng-xu

解决方案：

重写 \_start ，指定使用 os-ref/user/src/linker.ld 布局，编写 initcode.S、umain.c 文件并加入用户程序编译。

```makefile
LDFLAGS += -T ../../os-ref/user/src/linker.ld
LDFLAGS += -nostartfiles

runtest: src/common/runtest.obj $(COMMON_OBJS) initcode.S umain.c
	$(CC) $(LDFLAGS) $^ -static -o runtest.exe

static: $(COMMON_OBJS) $(STATIC_PREFIXED_OBJS) entry.c src/functional/tls_align_dso.obj initcode.S umain.c
	$(CC) $(LDFLAGS) -DSTATIC $^ -static -o entry-static.exe

dynamic: $(DYNAMIC_PREFIXED_OBJS) $(DSO_SOS) $(COMMON_OBJS) entry.c initcode.S umain.c
	$(CC) $(LDFLAGS) entry.c initcode.S umain.c -DDYNAMIC $(COMMON_OBJS) $(DYNAMIC_PREFIXED_OBJS) -Lsrc/functional -Lsrc/regression  -o entry-dynamic.exe -rdynamic
```

- 将 entry-static.exe、entry-dynamic.exe、runtest.exe 加入 fs.img

修改 os-ref/easy-fs-fuse/src/main.rs 。

./runos.sh 后可以看见程序正确加入。

根据 libc-test/run-static.sh 里的内容，测试一下：

```
>> runtest.exe -w entry-static.exe basename.exe
Unsupported syscall_id: 135, skip
Unsupported syscall_id: 135, skip
Unsupported syscall_id: 134, skip
========== START entry-static.exe basename.exe ==========
Unsupported syscall_id: 135, skip
Unsupported syscall_id: 135, skip
Unsupported syscall_id: 178, skip
[kernel] Segmentation Fault, SIGSEGV=11
Unsupported syscall_id: 137, skip
FAIL basename.exe [unknown]
========== END entry-static.exe basename.exe ==========
exit 1
```

测例已经可以被正确加载，后续可以开始实现 syscall 。

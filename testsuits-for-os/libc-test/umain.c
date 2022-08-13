#define MAX_ARGS 3
typedef long long int64_t;
typedef unsigned long long uint64_t;

#define SYS_write 64
#define SYS_exit 93

static inline int syscall(int64_t num, uint64_t a0, uint64_t a1, uint64_t a2) {
    int ret;

    asm volatile(
        "ld a7, %1\n"
        "ld a0, %2\n"
        "ld a1, %3\n"
        "ld a2, %4\n"
        "ecall\n"
        "sd a0, %0"
        : "=m"(ret)
        : "m"(num), "m"(a0), "m"(a1), "m"(a2)
        : "memory");
    return ret;
}

static inline int print(char* a, int len) {
    return syscall(SYS_write, 1, (uint64_t)a, (uint64_t)len);
}

static inline int sys_exit(int64_t error_code) {
    return syscall(SYS_exit, error_code, 0, 0);
}

int main(int argc, char** argv);

int strlen(char* s) {
    int i = 0;
    while (*s != 0) {
        ++i;
        ++s;
    }
    return i;
}

int umain(int argc, char** argv) {
    int ret = main(argc, argv);
    sys_exit(ret);
    while (1) {
    }
}

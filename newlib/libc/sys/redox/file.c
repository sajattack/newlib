#include "common.h"

int _unlink(const char *path) {
    return syscall2(SYS_UNLINK, (uint64_t)path, (uint64_t)strlen(path));
}

int _write(int file, const char *ptr, int len) {
    return syscall3(SYS_WRITE, (uint64_t)file, (uint64_t)ptr, (uint64_t)len);
}

int chmod(const char * path, mode_t mode) {
    return syscall3(SYS_CHMOD, (uint64_t)path, (uint64_t)strlen(path), (uint64_t)mode);
}

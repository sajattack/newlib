#include "common.h"

static char *curr_brk = NULL;

int _brk(void *end_data_segment) {
    char *new_brk;

    new_brk = __brk(end_data_segment);
    if (new_brk != end_data_segment) return -1;
    curr_brk = new_brk;
    return 0;
}

void * _sbrk(ptrdiff_t increment) {
    char *old_brk,*new_brk;

    if (!curr_brk) curr_brk = __brk(NULL);
    new_brk = __brk(curr_brk+increment);
    if (new_brk != curr_brk+increment) return (void *) -1;
    old_brk = curr_brk;
    curr_brk = new_brk;
    return old_brk;
}

int _sched_yield() {
    return syscall0(SYS_YIELD);
}

int _system(char * s){
    int pid = fork();
    if(pid == 0) {
        execl("/bin/sh", "-c", s, (char *)0);
        exit(100);
    } else if (pid < 0) {
        return -1;
    } else {
        int status = 0;
        int rc = waitpid(pid, &status, 0);
        if (rc < 0) {
            return -1;
        }
        return status;
    }
}

int _setregid(gid_t rgid, gid_t egid) {
    return syscall2(SYS_SETREGID, rgid, egid);
}

int _setreuid(uid_t ruid, gid_t euid) {
    return syscall2(SYS_SETREUID, ruid, euid);
}

pid_t _wait(int * status) {
    return waitpid(-1, status, 0);
}

pid_t waitpid(pid_t pid, int * status, int options) {
    return syscall3(SYS_WAITPID, (uint64_t)pid, (uint64_t)status, (uint64_t)options);
}

#include "common.h"

void _exit(int code){
    syscall1(SYS_EXIT, (uint64_t)code);
}

int _execve(const char *name, const char **argv, const char **env) {
    return syscall3(SYS_EXECVE, (uint64_t)name, (uint64_t)argv, (uint64_t)env);
}

int fork() {
    return syscall1(SYS_CLONE, 0);
}

char * getcwd(char * buf, size_t size) {
    char * cwd = NULL;

    int file = open(".", 0);
    if(file >= 0){
        if(!buf){
            if(size == 0){
                size = 4096;
            }
            buf = (char *)calloc(size, 1);

            if(fpath(file, buf, size) >= 0){
                cwd = buf;
            }else{
                free(buf);
            }
        }else{
            memset(buf, 0, size);
            if(fpath(file, buf, size) >= 0){
                cwd = buf;
            }
        }
        close(file);
    }

    return cwd;
}


pid_t getpid() {
    return syscall0(SYS_GETPID);
}

gid_t getegid() {
    return syscall0(SYS_GETEGID);
}

uid_t geteuid() {
    return syscall0(SYS_GETEUID);
}

gid_t getgid() {
    return syscall0(SYS_GETGID);
}

uid_t getuid() {
    return syscall0(SYS_GETUID);
}

int kill(int pid, int sig) {
    return syscall2(SYS_KILL, pid, sig);
}

void * __brk(void * addr) {
    return (void *)syscall1(SYS_BRK, (uint64_t)addr);
}

static char *curr_brk = NULL;

int brk(void *end_data_segment) {
    char *new_brk;

    new_brk = __brk(end_data_segment);
    if (new_brk != end_data_segment) return -1;
    curr_brk = new_brk;
    return 0;
}

void * sbrk(ptrdiff_t increment) {
    char *old_brk,*new_brk;

    if (!curr_brk) curr_brk = __brk(NULL);
    new_brk = __brk(curr_brk+increment);
    if (new_brk != curr_brk+increment) return (void *) -1;
    old_brk = curr_brk;
    curr_brk = new_brk;
    return old_brk;
}

int sched_yield() {
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

int setregid(gid_t rgid, gid_t egid) {
    return syscall2(SYS_SETREGID, rgid, egid);
}

int setreuid(uid_t ruid, gid_t euid) {
    return syscall2(SYS_SETREUID, ruid, euid);
}

pid_t wait(int * status) {
    return waitpid(-1, status, 0);
}

pid_t waitpid(pid_t pid, int * status, int options) {
    return syscall3(SYS_WAITPID, (uint64_t)pid, (uint64_t)status, (uint64_t)options);
}

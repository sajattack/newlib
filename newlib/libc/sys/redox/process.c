#include "common.h"

int chdir(const char *path){
    return syscall2(SYS_CHDIR, (uint64_t)path, (uint64_t)strlen(path));
}

void _exit(int code){
    syscall1(SYS_EXIT, (uint64_t)code);
}

int _execve(const char *name, const char **argv, const char **env) {
    // XXX Handle env
    int narg;
    for (narg=0; argv[narg] != NULL; narg++);
    char **argv2 = malloc(2 * narg * sizeof(char*));
    for (int i=0; i < narg; i++) {
        argv2[i*2] = argv[i];
        argv2[i*2+1] = strlen(argv[i]);
    }
    int ret = syscall4(SYS_EXECVE, (uint64_t)name, (uint64_t)strlen(name), (uint64_t)argv2, (uint64_t)narg);
    free(argv2);
    return ret;
}

int _fork() {
    return syscall1(SYS_CLONE, 0);
}

char * getcwd(char * buf, size_t size) {
    char * cwd = NULL;

    int file = _open(".", O_CLOEXEC | O_STAT, 0);
    if(file >= 0){
        if(!buf){
            if(size == 0){
                size = 4096;
            }
            buf = (char *)calloc(size, 1);

            if(_fpath(file, buf, size) >= 0){
                cwd = buf;
            }else{
                free(buf);
            }
        }else{
            memset(buf, 0, size);
            if(_fpath(file, buf, size) >= 0){
                cwd = buf;
            }
        }
        _close(file);
    }

    return cwd;
}

#ifndef MAXPATHLEN
#define MAXPATHLEN 1024
#endif

char * getwd(char *buf) {
    char tmp[MAXPATHLEN];

    if (buf == NULL) {
        errno = EINVAL;
        return NULL;
    }

    if (getcwd (tmp, MAXPATHLEN) == NULL) {
        return NULL;
    }

    return strncpy (buf, tmp, MAXPATHLEN);
}

pid_t _getpid() {
    return syscall0(SYS_GETPID);
}

gid_t _getegid() {
    return syscall0(SYS_GETEGID);
}

uid_t _geteuid() {
    return syscall0(SYS_GETEUID);
}

gid_t _getgid() {
    return syscall0(SYS_GETGID);
}

uid_t _getuid() {
    return syscall0(SYS_GETUID);
}

int _kill(int pid, int sig) {
    return syscall2(SYS_KILL, pid, sig);
}

void * __brk(void * addr) {
    return (void *)syscall1(SYS_BRK, (uint64_t)addr);
}

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

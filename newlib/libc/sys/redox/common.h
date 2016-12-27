/* note these headers are all provided by newlib - you don't need to provide them */
#include <sys/dirent.h>
#include <sys/errno.h>
#include <sys/fcntl.h>
#include <sys/stat.h>
#include <sys/times.h>
#include <sys/time.h>
#include <sys/types.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <errno.h>
#undef errno
extern int errno;

#define SYS_CLASS       0xF0000000
#define SYS_CLASS_PATH  0x10000000
#define SYS_CLASS_FILE  0x20000000

#define SYS_ARG         0x0F000000
#define SYS_ARG_SLICE   0x01000000
#define SYS_ARG_MSLICE  0x02000000
#define SYS_ARG_PATH    0x03000000

#define SYS_RET         0x00F00000
#define SYS_RET_FILE    0x00100000

#define SYS_LINK        SYS_CLASS_PATH | SYS_ARG_PATH | 9
#define SYS_OPEN        SYS_CLASS_PATH | SYS_RET_FILE | 5
#define SYS_CHMOD       SYS_CLASS_PATH | 15
#define SYS_RMDIR       SYS_CLASS_PATH | 84
#define SYS_UNLINK      SYS_CLASS_PATH | 10

#define SYS_CLOSE       SYS_CLASS_FILE | 6
#define SYS_DUP         SYS_CLASS_FILE | SYS_RET_FILE | 41
#define SYS_DUP2        SYS_CLASS_FILE | SYS_RET_FILE | 63
#define SYS_READ        SYS_CLASS_FILE | SYS_ARG_MSLICE | 3
#define SYS_WRITE       SYS_CLASS_FILE | SYS_ARG_SLICE | 4
#define SYS_LSEEK       SYS_CLASS_FILE | 19
#define SYS_FCNTL       SYS_CLASS_FILE | 55
#define SYS_FEVENT      SYS_CLASS_FILE | 927
#define SYS_FMAP        SYS_CLASS_FILE | 90
#define SYS_FUNMAP      SYS_CLASS_FILE | 91
#define SYS_FPATH       SYS_CLASS_FILE | SYS_ARG_MSLICE | 928
#define SYS_FSTAT       SYS_CLASS_FILE | SYS_ARG_MSLICE | 28
#define SYS_FSTATVFS    SYS_CLASS_FILE | SYS_ARG_MSLICE | 100
#define SYS_FSYNC       SYS_CLASS_FILE | 118
#define SYS_FTRUNCATE   SYS_CLASS_FILE | 93

#define SYS_BRK         45
#define SYS_CHDIR       12
#define SYS_CLOCK_GETTIME 265
#define SYS_CLONE       120
#define SYS_EXECVE      11
#define SYS_EXIT        1
#define SYS_FUTEX       240
#define SYS_GETCWD      183
#define SYS_GETEGID     202
#define SYS_GETENS      951
#define SYS_GETEUID     201
#define SYS_GETGID      200
#define SYS_GETNS       950
#define SYS_GETPID      20
#define SYS_GETUID      199
#define SYS_IOPL        110
#define SYS_KILL        37
#define SYS_MKNS        984
#define SYS_NANOSLEEP   162
#define SYS_PHYSALLOC   945
#define SYS_PHYSFREE    946
#define SYS_PHYSMAP     947
#define SYS_PHYSUNMAP   948
#define SYS_VIRTTOPHYS  949
#define SYS_PIPE2       331
#define SYS_SETREGID    204
#define SYS_SETRENS     952
#define SYS_SETREUID    203
#define SYS_WAITPID     7
#define SYS_YIELD       158

int64_t syscall0(uint64_t a);
int64_t syscall1(uint64_t a, uint64_t b);
int64_t syscall2(uint64_t a, uint64_t b, uint64_t c);
int64_t syscall3(uint64_t a, uint64_t b, uint64_t c, uint64_t d);
int64_t syscall4(uint64_t a, uint64_t b, uint64_t c, uint64_t d, uint64_t e);
int64_t syscall5(uint64_t a, uint64_t b, uint64_t c, uint64_t d, uint64_t e, uint64_t f);

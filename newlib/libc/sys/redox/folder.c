#include "common.h"

#include <sys/dirent.h>

DIR * _opendir(const char * path) {
    int fd = _open(path, O_RDONLY | O_CLOEXEC | O_DIRECTORY, 0);

    if(fd >= 0){
        DIR * dir = (DIR *)calloc(sizeof(DIR), 1);
        dir->dd_fd = fd;
        return dir;
    }

    return NULL;
}

struct dirent * _readdir(DIR * dir){
    if(dir){
        //TODO: Speed improvements
        int i;
        for(i = 0; i < 4096; ++i){
            if(_read(dir->dd_fd, &(dir->dd_ent.d_name[i]), 1) > 0){
                if(dir->dd_ent.d_name[i] == '\n'){
                    break;
                }
            }else{
                break;
            }
        }
        dir->dd_ent.d_name[i] = '\0';

        if(i > 0){
            return &(dir->dd_ent);
        }
    }

    return NULL;
}

void _rewinddir(DIR * dir){
    if(dir){
        _lseek(dir->dd_fd, 0, 0);
    }
}

int _closedir(DIR * dir){
    if(dir){
        _close(dir->dd_fd);
        free(dir);
        return 0;
    }

    return -1;
}

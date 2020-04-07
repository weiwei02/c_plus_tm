//
// Created by wangww on 2020/3/23.
//
#include <sys/stat.h>


int t_chmod(const char *path){
    return chmod(path, S_IXUSR | S_IXGRP | S_IXOTH);
}
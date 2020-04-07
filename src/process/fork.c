//#include <ntsid.h>
//#include <zconf.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

// fork 函数
// Created by wangww on 2020/3/8.
//
void tfork(){
    pid_t pid;
    if ((pid = fork()) < 0){
        printf("fork error \n");
        exit(1);
    } else if (pid == 0){
        printf("in the child process! \n");
    } else{
        printf("in the parent process! pid = %d\n", pid);
    }
    exit(0);
}



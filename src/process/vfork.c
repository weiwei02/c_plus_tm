//
// Created by wangww on 2020/3/8.
//
#include <unistd.h>
#include <stdio.h>
int gvar = 5;
void tvfork(){
    pid_t pid;
    int var = 5;
    printf("process id is %ld \n", (long)getpid());
    printf("gvar = %d, var = %d \n", gvar, var);

    // 创建一个新进程
    if ((pid = vfork()) < 0){
        perror("error !");
        return;
    } else if (pid == 0){
        gvar--;
        var++;
        printf("child process id is %ld \ngvar = %d, var = %d \n", (long)getpid(), gvar, var);
//        vfork 函数创建的子进程只能使用 _exit() 退出
        _exit(0);
    } else{
        printf("parent process id is %ld \ngvar = %d, var = %d \n", (long)getpid(), gvar, var);
    }
}
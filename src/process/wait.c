// wait 函数使用
// Created by wangww on 2020/3/21.
//
#include <process/process.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <signal.h>

void exit_s(int status);

void twait(){
    pid_t pid,pid1;
    int status;
    if ((pid = fork()) < 0){
        printf("child process error \n");
        exit(0);
    } else if (pid == 0){
        // 子进程
        printf("the child process \n");
    }
    if (wait(&status) != pid){
        // 在父进程中调用wait函数等待子进程结束
        printf("this is parent process \n wait error ! \n");
        exit(0);
    }
    // wait 函数调用成功，调用自定义函数，判断退出类型
    exit_s(status);

    // 再次创建子进程，子进程中使用kill函数发送信号，导致退出
    if ((pid = fork()) < 0){
        printf("child process error \n");
        exit(0);
    } else if (pid == 0){
        // 子进程
        printf("the child process \n");
        pid1 = getpid();
        // 使用kill函数发送信号
        // 结束进程
//        kill(pid1, 9);
        // 进入父进程
//        kill(pid1,17);
        // 终止进程
        kill(pid1,19);
    }
    if (wait(&status) != pid){
        // 在父进程中调用wait函数等待子进程结束
        printf("this is parent process \n wait error ! \n");
        exit(0);
    }

    // wait 函数调用成功，调用自定义函数，判断退出类型
    exit_s(status);
    exit(0);
}

// 判断子进程退出类型
void exit_s(int status) {
    if (WIFEXITED(status)){
        printf("normal exit,status = %d \n", WEXITSTATUS(status));
    } else if (WIFSIGNALED(status)){
        printf("signal exit ! status = %d\n", WTERMSIG(status));
    }
}

//
// Created by wangww on 2020/3/30.
//
#include <unistd.h>
#include <iostream>
#include <string>
//#include "include/ipc.h"

using namespace std;
#define MAXSIZE 100
int t_pipe(){
    int fd[2],line;
    pid_t pid, cpid = getpid();
    char message[MAXSIZE];
    string m = "this message send by pipe";

    // 创建管道
    if(pipe(fd) == -1){
        cerr << "create pipe failed!";
        return 1;
    }
    // 创建新进程
    else if((pid = fork()) < 0){
        cerr << "not create new process!";
        return 1;
    }
    // 子进程
    else if (pid == 0){
        close(fd[0]);
        cout << "child process " << getpid() << " send message\n";
        write(fd[1], m.data(), m.size());
        _exit(0);
    } else{
        // 如果不分别在父子进程关闭管道的出入口，可能会造成读取数据时无法获取到结束位置
        close(fd[1]);
        cout << "parent process " << cpid << " receive message is:\n";
        line = read(fd[0], message, MAXSIZE);
        write(STDOUT_FILENO, message, line);
        cout << endl;
        // 等待开启的但未结束的子进程
        wait(NULL);
    }
    return 0;
}
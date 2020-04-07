#include <sys/types.h>
#include <sys/stat.h>
#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
// 内核文件操作
// Created by wangww on 2020/3/29.
// 使用文件描述符做文件操作
const char * path = "/Users/wangww/CLionProjects/tm/cmake-build-debug/test/resources/tfile.txt";
int tKernelFile(){
    int fd;
    // 自定义读写用的缓冲区
    char buf[1024], buf2[] = "hello world";
    int n,i;
    if((fd = open(path, O_CREAT | O_RDWR | O_APPEND, 0666)) < 0){
        perror("open file failed!");
        return 1;
    } else{
        printf("open file successful\n");
    }

    if ((n = read(fd, buf, 100)) < 0){
        perror("read failed!");
        return 1;
    } else{
        printf("output read data:\n");
        printf("%s\n", buf);
    }

    // 定位到从文件开头的11个字符
    if ((i = lseek(fd, 100, SEEK_SET))){
        perror("lseek error!");
        return 1;
    } else{
        if (write(fd, buf2, 11) < 0){
            perror("write error!");
            return 1;
        } else{
            printf("write successful!\n");
        }
    }
    // 关闭文件的同时保存文件改动
    close(fd);


    if((fd = open(path, O_RDWR)) < 0){
        perror("open file 2 failed!");
        return 1;
    } else{
        printf("open file 2 successful\n");
    }
    if ((n = read(fd, buf, 120)) < 0){
        perror("read 2 failed!");
        return 1;
    } else{
        printf("read the changed data:\n");
        printf("%s\n", buf);
    }
    if (close(fd) < 0){
        perror("close failed!");
        return 1;
    } else{
        printf("good bye!\n");
    }
    return 0;
}


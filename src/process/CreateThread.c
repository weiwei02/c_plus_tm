//
// Created by wangww on 2020/3/29.
//
#include <pthread.h>
#include <stdio.h>
#include <zconf.h>
#include <string.h>
#include <stdlib.h>

char message[] = "Hello Pthread";
void *thread_function(void *args){
    printf("thread function is running ,argument was %s\n", (
    char*) args);
    sleep(3);
    strcpy(message, "Bye!");
    pthread_exit("Thank you for the CPU times");
}
// 测试线程创建
int tcreatThread(){
    int res;
    // 声明局部变量时，编译器自动的分配了栈内存地址。
    pthread_t a_thread;
    void *thread_result;
    res = pthread_create(&a_thread, NULL, thread_function, (void *)message);
    if (res != 0){
        perror("Thread creation failed!");
        exit(EXIT_FAILURE);
    }
    printf("Waiting for thread to finish...\n");
    res = pthread_join(a_thread, &thread_result);
    if (res != 0){
        perror("Thread join failed!");
        exit(EXIT_FAILURE);
    }

    printf("Thread joined ,it returned %s\n", (
    char *) thread_result);
    printf("Message now is %s", message);
    exit(EXIT_SUCCESS);
}
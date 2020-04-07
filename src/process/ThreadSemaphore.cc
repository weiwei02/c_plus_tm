// 线程信号量同步
// Created by wangww on 2020/3/29.
//
#include <iostream>
#include <pthread.h>
#include <semaphore.h>
#include <string>
#include <cstring>
using namespace std;
sem_t bin_sem;
void *thread_function1(void *args);

#define WORD_SIZE 1024
extern char work_area[WORD_SIZE];

int t_smaphore(){
    int res;
    pthread_t a_thread;
    void *thread_result;

    res = sem_init(&bin_sem, 0, 0);
    if (res < 0){
        perror("init semaphore failed");
        return res;
    }
    res = pthread_create(&a_thread, nullptr, thread_function1, nullptr);
    if (res != 0){
        perror("Thread creation failed");
        return res;
    }
    cout << "Input some text, Enter 'end' to finish \n";
    while (strcmp(work_area, "end") != 0){
        cin.getline(work_area, WORD_SIZE);
        sem_post(&bin_sem);
    }
    cout << endl << "Waiting for thread to finish...\n";
    res = pthread_join(a_thread, &thread_result);
    if(res != 0){
        perror("Thread join failed");
        return res;
    }
    cout << "Thread join\n";
    sem_destroy(&bin_sem);
    return 0;
}

void *thread_function1(void *args){
    sem_wait(&bin_sem);
    while (strcmp(work_area, "end") != 0){
        cout << "You input " << strlen(work_area) - 1 <<" characters" << endl;
        sem_wait(&bin_sem);
    }
    pthread_exit(nullptr);
}
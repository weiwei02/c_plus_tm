// 线程同步
// Created by wangww on 2020/3/29.
//
#include <iostream>
#include <pthread.h>
#include <string>
#include <cstring>
#include <zconf.h>

using namespace std;
void *thread_function(void *args);

pthread_mutex_t work_mutex;
int time_to_exit = 0;
#define WORD_SIZE 1024
char work_area[WORD_SIZE];
const string end = "end";

int t_thread_mutex() {
    int res;
    pthread_t a_thread;
    void *thread_result;
    res = pthread_mutex_init(&work_mutex, nullptr);
    if (res != 0){
        cerr << "Mutex initialization failed";
        return res;
    }
    res = pthread_create(&a_thread, nullptr, thread_function, nullptr);
    if (res != 0){
        perror("Thread creation failed");
        return res;
    }
    pthread_mutex_lock(&work_mutex);
    cout << "Input some text, Enter 'end' to finish \n";
    while (!time_to_exit){
        cin.getline(work_area, WORD_SIZE);
        pthread_mutex_unlock(&work_mutex);
        while (1){
            pthread_mutex_lock(&work_mutex);
            if (work_area[0] != '\0'){
                pthread_mutex_unlock(&work_mutex);
                sleep(1);
            } else{
                break;
            }
        }
    }
    pthread_mutex_unlock(&work_mutex);
    cout << endl << "Waiting for thread to finish...\n";
    res = pthread_join(a_thread, &thread_result);
    if(res != 0){
        perror("Thread join failed");
        return res;
    }
    cout << "Thread join\n";
    pthread_mutex_destroy(&work_mutex);
    return 0;
}

void *thread_function(void *args){
    sleep(1);
    pthread_mutex_lock(&work_mutex);
    while (strcmp(work_area, "end") != 0){
        cout << "You input " << strlen(work_area) <<" characters" << endl;
        work_area[0] = '\0';
        pthread_mutex_unlock(&work_mutex);
        sleep(1);
        pthread_mutex_lock(&work_mutex);
        while (work_area[0] == '\0'){
            pthread_mutex_unlock(&work_mutex);
            sleep(1);
            pthread_mutex_lock(&work_mutex);
        }
    }

    time_to_exit = 1;
    work_area[0] = '\0';
    pthread_mutex_unlock(&work_mutex);
    pthread_exit(0);
}
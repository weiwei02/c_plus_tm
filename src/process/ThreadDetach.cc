// 线程分离测试
// Created by wangww on 2020/3/29.
//pthread_join() 子程序阻碍调用程序，直到指定的 threadid 线程终止为止。
// 当创建一个线程时，它的某个属性会定义它是否是可连接的（joinable）或可分离的（detached）。
// 只有创建时定义为可连接的线程才可以被连接。如果线程创建时被定义为可分离的，则它永远也不能被连接。
//
//这个实例演示了如何使用 pthread_join() 函数来等待线程的完成。
#include <iostream>
#include <cstdlib>
#include <pthread.h>
#include <unistd.h>

using namespace std;

#define NUM_THREADS     5

void *wait(void *t)
{
    int i;
    long tid;

    tid = (long)t;

    sleep(1);
    cout << "Sleeping in thread " << endl;
    cout << "Thread with id : " << tid << "  ...exiting " << endl;
    pthread_exit(NULL);
}

int t_thread_detach ()
{
    int rc;
    int i;
    pthread_t threads[NUM_THREADS];
    pthread_attr_t attr;
    void *status;

    // 初始化并设置线程为可连接的（joinable）
    pthread_attr_init(&attr);
    // 如果设置为 PTHREAD_CREATE_JOINABLE，就继续用 pthread_join() 来等待和释放资源，否则会内存泄露。
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_JOINABLE);

    for( i=0; i < NUM_THREADS; i++ ){
        cout << "main() : creating thread, " << i << endl;
        rc = pthread_create(&threads[i], NULL, wait, (void *)&i );
        if (rc){
            cout << "Error:unable to create thread," << rc << endl;
            return 1;
        }
    }

    // 删除属性，并等待其他线程
    pthread_attr_destroy(&attr);
    for( i=0; i < NUM_THREADS; i++ ){
        rc = pthread_join(threads[i], &status);
        if (rc){
            cout << "Error:unable to join," << rc << endl;
            exit(-1);
        }
        cout << "Main: completed thread id :" << i ;
        cout << "  exiting with status :" << status << endl;
    }

    cout << "Main: program exiting." << endl;
    return 0;
}


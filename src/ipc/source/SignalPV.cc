#include <sys/sem.h>
#include <stdlib.h>
#include <unistd.h>

#include <iostream>

using namespace std;
static int sem_id;
int set_semvalue();
int del_semvalue();
int semaphor_p();
int semaphor_v();

int t_sigpv(int mode){
    int pause_time;
    char op_char = '0';
    srand((unsigned int)getpid());

    sem_id = semget((key_t)1234, 1, 0666 | IPC_CREAT);

    if (mode > 1)
    {
        if (!set_semvalue())
        {
            cout << "Failed to initialize semaphor \n";
            op_char = 'X';
            sleep(2);
        }
        
    }
    //循环，它进入和离开临界区域10次。在每次循环的开始，首先调用semaphore_p函数，它在程序将进入临界区域时设置信号量以等待进入
    for (int i = 0; i < 10; i++)
    {
        if (!semaphor_p()) 
        {
            return EXIT_FAILURE;
        }
        cout << op_char << endl;
        sleep(rand()%3);
        cout << op_char << endl;

        // “在临界区域之后，调用semaphore_v来将信号量设置为可用，然后等待一段随机的时间，再进入下一次循环。”
        if (!semaphor_v()) 
        {
            return EXIT_FAILURE;
        }
        sleep(rand()%3);
    }
    cout << getpid() << " finished \n";
    if (mode > 1)
        {
            sleep(10);
            del_semvalue();
        }
    return EXIT_SUCCESS;
}

/**
 * “通过将semctl调用的command参数设置为SETVAL来初始化信号量。在使用信号量之前必须要这样做”
*/
int set_semvalue(){
    union semun sem_union;
    sem_union.val = 1;
    if(semctl(sem_id, 0, SETVAL, sem_union)) return 0;
    return 1;
}

/**
 * “函数del_semvalue的形式与上面的函数几乎一样，只不过它通过将semctl调用的command设置为IPC_RMID来删除信号量ID”
*/
int del_semvalue(){
    union semun sem_union;
    if(semctl(sem_id, 0, IPC_RMID, sem_union)) return 0;
    return 1;
}

/**“semaphore_p对信号量做减1操作（等待）”
 * 
*/
int semaphor_p(){
    struct sembuf sem_b = {
        0, -1, SEM_UNDO
    };
    if (semop(sem_id, &sem_b, 1) == -1)
    {
        cout << "semaphor_p failed"<< endl;
        return 0;
    }
    return 1;
}

int semaphor_v(){
struct sembuf sem_b = {
        0, 1, SEM_UNDO
    };
    if (semop(sem_id, &sem_b, 1) == -1)
    {
        cout << "semaphor_v failed"<< endl;
        return 0;
    }
    return 1;
}
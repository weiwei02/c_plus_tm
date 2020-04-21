#include <time.h>
#include <stdlib.h>
#include <unistd.h>
#include <poll.h>
#include <iostream>

using namespace std;
/**
 * 创建一些传导（每个管道使用一对连续的文件描述符），将数据写到随机选择的管道的写端，
 * 然后通过poll来检查哪个管道中有可读数据。
*/
int poll_pipes(int pipes_num,int wirte_num){
    
    
    // 申请同样数量个poll文件描述符
    void * _pfds = calloc(pipes_num, sizeof(struct pollfd));
    if (_pfds == NULL)
    {
        cerr << "malloc error \n";
        return -1;
    }
    struct pollfd *poll_fds = (struct pollfd *)_pfds;


    void * pfds_ = calloc(pipes_num, sizeof(int [2]));
    if (pfds_ == NULL)
    {
        cerr << "malloc error \n";
        return -1;
    }
    // 所有管道的文件描述符，0读1写
    int (*pfds)[2] = (int (*) [2])pfds_;

    cout << "创建接口声明数量的管道 \n";
    for (int i = 0; i < pipes_num; i++)
    {
        if (pipe(pfds[i]) == -1)
        {
            cerr << "create pipe " << i << " error\n";
            return -1;
        }
    }

    cout << "执行向指定数量随机选择的管道中写数据\n";
    srandom((int)time(NULL));
    for (int i = 0; i < wirte_num; i++)
    {
        int rand_pipe = random() % wirte_num;
        cout << "write to fd " << rand_pipe << " " << pfds[rand_pipe][1] << " \n";
        if (write(pfds[rand_pipe][1], "abc", 3) == -1)
        {
            cerr << "write to " << rand_pipe << " " << pfds[rand_pipe][1] << " error\n";
        }
    }

    cout << "构造poll函数所需的文件描述符\n";
    for (int j = 0; j < pipes_num; j++)
    {
        poll_fds[j].fd = pfds[j][0];
        poll_fds[j].events = POLL_IN;
    }
    
    int ready = poll(poll_fds, pipes_num, -1);
    if (ready == -1)
    {
        cerr << "poll 函数执行出错\n";
    }
    cout << "poll() return " << ready << " \n";

    cout << "检查每个管道是否有可用消息到达\n";
    for (int k = 0; k < pipes_num; k++)
    {
        if (poll_fds[k].revents & POLL_IN)
        {
            cout << "readable " << k << " " << poll_fds[k].fd << endl;
        }
        
    }
    return 0;
}
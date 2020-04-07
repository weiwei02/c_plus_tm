// 线程间通信测试
// Created by wangww on 2020/3/30.
//

#ifndef TM_IPC_H
#define TM_IPC_H

/**
 * #include <unistd.h>
 * int pipe()int filedes[2];
 *
 * 函数用于在内核中创建一个管道，该管道一端用于读取数据，另一端用于写入数据。
 * 在创建管道后，会获取一对文件描述符，用于读取和写入，然后将参数数组 fildes[0] 指向读端，filedes[1] 指向写端
 * pipe()函数调用成功返回0；否则返回-1，并且设置了相关的错误信息。
 * DEFAULT: 参数filedes非法
 * EMFILE: 进程中使用了过多的文件描述符
 * ENFILE: 打开的文件达到了系统允许的最大值
 *
 * pipe只允许两个有关系的进程之间进行通信
 * pipe在linux中并不支持双向通信
 * */
int t_pipe();
#endif //TM_IPC_H

// 测试线程同步
// Created by wangww on 2020/3/30.
//
#include <process/threads.h>
#include <iostream>
int main()
{
    std::cout << "测试线程detach。 返回值： " << t_thread_detach();
}

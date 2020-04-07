//
// Created by wangww on 2020/3/7.
//

#ifndef TM_MEM_H
#define TM_MEM_H

#endif //TM_MEM_H

#include <stdio.h>
#include <mm_malloc.h>
//#include <malloc/malloc.h>
#define LEN sizeof(struct Student)

// 测试对某个指针的已分配的内存空间重新做分配
int drealloc();

// 批量重新设置内存地址的值为某个字符
void tmemset();

// 链表使用的数据结构
struct Student{
    int num;
    int age;
    float score;
    struct Student *next;
};

typedef struct Student stu;

// 创建链表
stu * create();
#include <mem/mem.h>
#include <mm_malloc.h>

// 链表应用测试
// Created by wangww on 2020/3/7.
//
// 创建链表
stu * create(){
    // 定义结构体类型的指针
    stu *head = NULL,*p1,*p2;
    int n = 0;
    // 开辟一个内存空间
    p1 = p2 = (stu *)malloc(LEN);
    // 输入结构体类型的指针
    scanf("%d,%d,%f", &p1->num, &p1->age, &p1->score);

    // 判断学号是否为0，如果不为0就继续输入
    while (p1->num != 0){
        n = n + 1;
        if (n == 1){
            head = p1;
        } else{
            p2->next = p1;
        }
        p2 = p1;
        p1 = (stu *)malloc(LEN);
    }
    return head;
}


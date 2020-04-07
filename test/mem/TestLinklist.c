// 测试链表调用
// Created by wangww on 2020/3/7.
//
#include <mem/mem.h>
int main(){
    stu *p, *head;
    head = create();
    p = head;
    if (head != NULL){
        do{
            printf("%d,%d,%f\n", p->num, p->age, p->score);
            p = p->next;
        }while (p != NULL);
    }
}


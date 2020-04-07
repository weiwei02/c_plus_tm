#include <stdlib.h>
#include <stdio.h>

// realloc 内存管理
// Created by wangww on 2020/3/7.
//
int drealloc(){
    char* p;
    p = (char *)malloc(100);
    if (p) {
        // 内存分配成功
        printf("Memory Allocate at %s \n", p);
//        printf("Memory Allocate at %x \n", p);
    }else{
        printf("No Enough Memory \n");
    }
    // 调整p内存空间从100 到256
    p = realloc(p, 256);
    if (p) {
        printf("Memory Reallocated at %s\n",p);
//        printf("Memory Reallocated at %x\n",p);
    } else{
        printf("No Enough Memory\n");
    }

    // 释放内存空间
    free(p);
    return 0;
}
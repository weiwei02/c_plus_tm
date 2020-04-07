// memset 批量的将某个字符写到指定的内存块里
// Created by wangww on 2020/3/7.
//

#include <string.h>
#include <stdio.h>

void tmemset(){
    char s[] = "welcome to unix\n";
    printf("s before memset: %s", s);
    memset(s, '*', strlen(s) - 1);
    printf("s after memset: %s", s);
}
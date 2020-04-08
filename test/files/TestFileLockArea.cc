// 测试文件区域锁
// Created by wangww on 2020/4/7.
//
#include "kernel_files.h"
#include <iostream>

int main(){
    std::cout << "hellogwold";
    std::cout << "hellogwold";
    lock_file_area();
}
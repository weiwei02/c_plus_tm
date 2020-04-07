// 测试c++ 面向对象编程
// Created by wangww on 2020/3/28.
//
#include <cxx/Include/stock10.h>
#include <iostream>

int main(){
    using std::cout;
    cout << "Using constructors to create new objects\n";
    Stock stock1("NanoSmart", 12, 20.0);
    stock1.show();

    Stock stock2 = Stock("Boffo Objects", 2, 2.0);
    stock2.show();

    cout << "Done\n";
}

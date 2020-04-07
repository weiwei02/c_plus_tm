// 测试右值移动
// Created by wangww on 2020/4/6.
//
#include <iostream>

#include "gtest/gtest.h"
#include "include/Stdmove.h"
int t_move(){
    using std::cout;
    {
        UseLess one(10, 'x');
        UseLess two = one + one;
        cout << "Object one : ";
        one.ShowData();
        cout << "Object two : ";
        two.ShowData();

        UseLess three, four;
        cout << "Three = one \n";
        three = one;
        cout << "Now object three = ";
        three.ShowData();

        cout << "four = one + two\n";
        four = one + two;
        cout << "now object four = ";
        four.ShowData();

        cout << "four = move(one) :\n";
        four = std::move(one);
        cout << "now object four = ";
        four.ShowData();

        cout << "and object one = ";
        one.ShowData();
    }
    return 0;
}

TEST(测试右值, move函数1){
    ASSERT_EQ(t_move(), 0);
}


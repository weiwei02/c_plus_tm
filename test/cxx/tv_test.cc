// 测试tv友元类使用
// Created by wangww on 2020/4/5.
//

#include <iostream>
#include "gtest/gtest.h"
#include "Include/Tv.h"


bool T_tv(){
    using std::cout;
    Tv s42;
    cout << "Initial setting for 42\"tv:\n";
    s42.settings();
    s42.onOff();
    s42.chanUp();
    cout << "\"Adjusted settings for 42 \"TV: \n";
    s42.settings();

    Remote grey;

    grey.setChan(s42, 10);
    grey.volUp(s42);
    grey.volUp(s42);
    cout << "\n 42 \" settings after using remote :\n";
    s42.settings();

    Tv s58(Tv::On);
    s58.setMode();
    grey.setChan(s58, 28);
    cout << "\n58\" settings :\n";
    s58.settings();
    return s58.isOn();
}


TEST(FactorialTest, Negative){
    ASSERT_TRUE(T_tv()) << "电视状态不是开启";
//    ASSERT_EQ(101, 100);
//    ;
//    ASSERT_EQ("asdsa", "asd");
}

int main(){

}
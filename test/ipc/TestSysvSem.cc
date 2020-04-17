// 测试system v 信号量
#include "gtest/gtest.h"
#include "include/sysv.h"

TEST(systemv, semaphor){
    if (fork() == 0)
    {
        EXPECT_EQ(0,t_sigpv(2));
    }else
    {
        EXPECT_EQ(0,t_sigpv(0));
    } 
}
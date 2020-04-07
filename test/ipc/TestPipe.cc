// 管道测试
// Created by wangww on 2020/4/4.
//

#include "gtest/gtest.h"
#include "include/ipc.h"

TEST(HelloTest, GetGreet) {
    EXPECT_EQ(t_pipe(), 0);
}
// 半同步线程池测试
// Created by wangww on 2020/4/7.
//
#include "ThreadPool.h"
void testPool(){
    ThreadPool pool;
    pool.Start(2);
    std::thread thd1([&pool]{
        for (int i = 0; i < 10; ++i) {
            auto thId = this_thread::get_id();
            pool.AddTask([thId]{
                cout << "同步线程1的线程id： " << thId << endl;
            });
        }
    });
    std::thread thd2([&pool]{
        for (int i = 0; i < 10; ++i) {
            auto thId = this_thread::get_id();
            pool.AddTask([thId]{
                cout << "同步线程2的线程id： " << thId << endl;
            });
        }
    });

    this_thread::sleep_for(std::chrono::seconds(2));
    getchar();
    thd1.join();
    thd2.join();
}

int main(){
    testPool();
}


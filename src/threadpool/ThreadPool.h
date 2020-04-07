//
// Created by wangww on 2020/4/7.
//

#ifndef TM_THREADPOOL_H
#define TM_THREADPOOL_H

#include "SyncQueue.h"

const int MaxTaskCount = 100;
class ThreadPool {
public:
    // 未命名函数写法，与c的函数指针有所不同
    // void (*run)();
    using Task = std::function<void ()>;

    ThreadPool(int num = std::thread::hardware_concurrency()) : m_queue(MaxTaskCount){
        Start(num);
    }

    void Stop() {
        // 保证多线程环境下只调用一次stopThreadGroup
        std::call_once(m_flag, [this]{StopThreadGroup();});
    }

    ~ThreadPool(){
        // 如果没有停止时应主动停止线程池
        Stop();
    }

    void AddTask(Task &&task){
        m_queue.Put(std::forward<Task >(task));
    }
    void AddTask(const Task& task){
        m_queue.Put(task);
    }

    void Start(int num) {
        m_running = true;
        // 创建线程组
        for (int i = 0; i < num; ++i) {
            m_threadgroup.push_back(std::make_shared<std::thread>(&ThreadPool::RunInThread, this));
        }
    }

private:
    std::list<std::shared_ptr<std::thread>> m_threadgroup;
    // 处理任务的线程组
    SyncQueue<Task> m_queue;
    // 是否停止标志
    atomic_bool m_running;
    std::once_flag m_flag;





    void StopThreadGroup() {
        m_queue.Stop();
        m_running = false;
        for (const auto& thread1 : m_threadgroup){
            if (thread1)
                thread1->join();
        }
        m_threadgroup.clear();
    }

    void RunInThread(){
        while (m_running){
            // 取任务分别执行
            std::list<Task > list;
            m_queue.Take(list);
            for (auto &task : list){
                if (!m_running)
                    return;
                task();
            }
        }
    }
};


#endif //TM_THREADPOOL_H

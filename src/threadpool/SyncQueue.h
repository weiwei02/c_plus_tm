// 同步队列定义
// Created by wangww on 2020/4/2.
//

#ifndef TM_SYNCQUEUE_H
#define TM_SYNCQUEUE_H

#include <list>
#include <mutex>
#include <thread>
#include <condition_variable>
#include <iostream>
using namespace std;
template<typename T>
class SyncQueue {
private:
    // 缓冲队列
    std::list<T> m_queue;
    // 互斥量，和条件变量结合使用
    std::mutex m_mutex;
    // 不为空的条件变量
    std::condition_variable m_notEmpty;
    // 队列未满条件变量
    std::condition_variable m_notFull;
    // 队列最大size
    int m_maxSize;
    // 停止标志
    bool m_needStop;
public:
    SyncQueue(int maxSize) :m_maxSize(maxSize), m_needStop(false){}
    void Put(const T&x){Add(x);}

    void Put(T&&x){Add(std:: forward<T>(x));}

    /**
     * 通过一次加锁，将队列中的所有数据都取出来，大大减少了加锁的次数
     * 同时有通过move函数减少内存复制，提升性能
     * */
    void Take(std::list<T> &list){
        unique_lock<mutex> locker(m_mutex);
        m_notEmpty.wait(locker, [this]{ return m_needStop || NotEmpty();});
        if (m_needStop)
            return;
        list = std::move(m_queue);
        m_notFull.notify_one();
    }

    void Take(T& t){
        std::unique_lock<std::mutex> locker(m_mutex);
        m_notEmpty.wait(locker, [this]{return m_needStop || NotEmpty();});
        if (m_needStop)
            return;
        t = m_queue.front();
        m_queue.pop_front();
        m_notFull.notify_one();
    }

    void Stop()
    {
        {
            std::lock_guard<std::mutex> locker(m_mutex);
            m_needStop = true;
        }
        m_notFull.notify_all();
        m_notEmpty.notify_all();
    }

    bool Empty(){
        std::lock_guard<std::mutex> locker(m_mutex);
        return m_queue.empty();
    }

    bool Full(){
        std::lock_guard<std::mutex> locker(m_mutex);
        return m_queue.size() == m_maxSize;
    }

    size_t Size(){
        std::lock_guard<std::mutex> locker(m_mutex);
        return m_queue.size();
    }

    int Count(){
        return m_queue.size();
    }

private:
    bool NotFull(){
        bool full = m_queue.size() >= m_maxSize;
        if (full)
            cout << "缓冲区满了，需要等待..." << endl;
        return !full;
    }

    bool NotEmpty(){
        bool empty = m_queue.empty();
        if (empty)
            cout << "缓冲区空了，需要等待...异步线程id： " << this_thread::get_id() << endl;
        return !empty;
    }

    /**
     * add过程：
     * 先获取mutex，然后检查条件是否满足。不满足条件时，释放，mutex继续等待。
     * 如果满足条件，则将新任务插入到队列中，并唤醒取任务的线程取取数据
     * */
    template <typename F>
    void Add(F&&x){
        std::unique_lock<std::mutex> locker(m_mutex);
        m_notFull.wait(locker, [this]{ return m_needStop || NotFull();});
        if (m_needStop)
            return;
        m_queue.push_back(std:: forward<F>(x));
        m_notEmpty.notify_one();
    }
};

#endif //TM_SYNCQUEUE_H

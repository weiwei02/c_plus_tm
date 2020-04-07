//
// Created by wangww on 2020/3/28.
//

#ifndef TM_STOCK10_H
#define TM_STOCK10_H

#include <string>

class Stock{
private:
    std::string company;
    long shares;
    double share_val;
    double total_val;
    void set_tot(){total_val = share_val * share_val;}

public:
    // two constructors
    Stock();
    Stock(const std::string & cc, long n = 0, double pr = 0.0);
    // noisy destructor
    ~Stock();
    void buy(long num, double price);
    void sell(long num, double price);
    void update(double price);
    void show();
    // 比较两个对象哪个股价更高，返回股价更高的对象的引用
    // 括号后的const表明，该函数不会修改被隐式访问的对象
    const Stock & topval(const Stock &) const;
};
#endif //TM_STOCK10_H

//
// Created by wangww on 2020/4/6.
//

#include "include/Stdmove.h"
#include <iostream>

int UseLess::ct = 0;

UseLess::UseLess()
{
    ++ct;
    n = 0;
    pc = nullptr;
}

UseLess::UseLess(int k):n(k) {
    ++ct;
    pc = new char[n];
}

UseLess::UseLess(int k, char ch) :n(k){
    ++ct;
    pc = new char[n];
    for (int i = 0; i < n; ++i) {
        pc[i] = ch;
    }
}

UseLess::UseLess(const UseLess &t): n(t.n) {
    ++ct;
    pc = new char[n];
    for (int i = 0; i < n; ++i) {
        pc[i] = t.pc[i];
    }
}

UseLess::UseLess(UseLess &&t) : n(t.n) {
    ++ct;
    pc = t.pc;
    t.pc = nullptr;
    t.n = 0;
}

UseLess ::~UseLess() {
    delete [] pc;
}

UseLess& UseLess::operator=(const UseLess &f) {
    std::cout << "copy assignment operator called;\n";
    if (this == &f)
        return *this;
    delete []pc;
    n = f.n;
    pc = new char (n);
    for (int i = 0; i < n; ++i) {
        pc[i] = f.pc[i];
    }
    return *this;
}

UseLess & UseLess::operator=(UseLess &&f) {
    std::cout << "move assignment operator called;\n";
    if (this == &f)
        return *this;
    delete []pc;
    n = f.n;
    pc = f.pc;
    f.n = 0;
    f.pc = nullptr;
    return *this;
}

UseLess UseLess::operator+(const UseLess &f) const {
    UseLess temp = UseLess(n + f.n);
    for (int i = 0; i < n; ++i) {
        temp.pc[i] = pc[i];
    }
    for (int j = n; j < temp.n; ++j) {
        temp.pc[j] = f.pc[j - n];
    }
    return temp;
}

void UseLess::showObject() const {
    std::cout << "Number og element: " << n ;
    std::cout << " data address: " << (void *)pc << std::endl;
}
void UseLess::ShowData() const {
    if (n == 0)
        std::cout << "(Object empty)";
    else
        for (int i = 0; i < n; ++i) {
            std::cout << pc[i];
        };
        std::cout << std::endl;
}
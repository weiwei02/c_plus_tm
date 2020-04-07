//
// Created by wangww on 2020/4/6.
//

#ifndef TM_STDMOVE_H
#define TM_STDMOVE_H


class UseLess {
private:
    // number of elements
    int n;
    // point of data
    char * pc;
    // number of object
    static int ct;
    void showObject() const ;

public:
    UseLess();
    explicit UseLess(int k);
    UseLess(int k , char ch);
    // regular copy constructor
    UseLess(const UseLess &t);
    // move constructor
    UseLess(UseLess &&t);
    ~UseLess();

    UseLess operator+(const UseLess &f) const ;
    // copy
    UseLess &operator=(const UseLess &f);
    //move
    UseLess &operator=(UseLess &&f);
    void ShowData() const ;
};


#endif //TM_STDMOVE_H

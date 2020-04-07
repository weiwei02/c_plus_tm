//
// Created by wangww on 2020/4/1.
//

#ifndef TM_DMA_H
#define TM_DMA_H

#include <iostream>

class BaseDMA {
private:
    char  * lable;
    int rating;
public:
    BaseDMA(const char * l = "null", int r = 0);
    BaseDMA(const BaseDMA & rs);
    virtual ~BaseDMA();
    BaseDMA &operator=(const BaseDMA & rs);
    friend std::ostream &operator<<(std::ostream & os, const BaseDMA & rs);
};

// derived class without DMA
// no destructor needed
// uses implicit copy constructor
// uses implicit assignment operator
class LacksDMA:public BaseDMA
{
private:
    enum {COL_LEN = 40};
    char color[COL_LEN];
public:
    LacksDMA(const char *c = "blank", const char * l = "null", int r = 0);
    LacksDMA(const char *c, const BaseDMA & rs);
    friend std::ostream &operator<<(std::ostream & os, const LacksDMA & rs);
};

// derived class with DMA
class HasDMA:public BaseDMA
{
private:
    char * style;
public:
    HasDMA(const char *c = "none", const char * l = "null", int r = 0);
    HasDMA(const char * s, const BaseDMA & rs);
    HasDMA(const HasDMA & hs);
    ~HasDMA();
    HasDMA &operator=(const HasDMA & rs);
    friend std::ostream &operator<<(std::ostream & os, const HasDMA & rs);
};

#endif //TM_DMA_H

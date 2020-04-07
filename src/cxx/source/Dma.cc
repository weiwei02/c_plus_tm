//
// Created by wangww on 2020/4/1.
//

#include "include/Dma.h"
#include <cstring>
using namespace std;
// baseDMA methods
BaseDMA::BaseDMA(const char *l, int r) {
    lable = new char[strlen(l) + 1];
    strcpy(lable, l);
    rating = r;
}

// copy object method
BaseDMA::BaseDMA(const BaseDMA &rs) {
    cout << "base dma copy()";
    lable = new char[strlen(rs.lable) + 1];
    strcpy(lable, rs.lable);
    rating = rs.rating;
}

BaseDMA::~BaseDMA() {
    delete [] lable;
}
BaseDMA & BaseDMA::operator=(const BaseDMA &rs) {
    if (this == &rs)
        return *this;
    delete [] lable;
    lable = new char [strlen(rs.lable) + 1];
    strcpy(lable, rs.lable);
    rating = rs.rating;
    return *this;
}

std::ostream& operator<<(std::ostream &os, const BaseDMA &rs) {
    os << "Label: " << rs.lable << endl;
    os << "Rating: " << rs.rating << endl;
    return os;
}

// LacksDMA methods
LacksDMA::LacksDMA(const char *c, const char *l, int r) :BaseDMA(l, r){
    strncpy(color, c, 39);
    color[39] = '\0';
}

LacksDMA::LacksDMA(const char *c, const BaseDMA &rs) :BaseDMA(rs){
    strncpy(color, c, COL_LEN - 1);
    color[COL_LEN - 1] = '\0';
}

std::ostream& operator<<(std::ostream &os, const LacksDMA &rs) {
    os << (const BaseDMA &)rs;
    os << "Color: " << rs.color << endl;
    return os;
}

HasDMA::HasDMA(const char *c, const char *l, int r)
:   BaseDMA(l, r)
{
    style = new char [strlen(l) + 1];
    strcpy(style, l);
}

HasDMA::HasDMA(const char *s, const BaseDMA &rs)
:   BaseDMA(rs)
{
    style = new char [strlen(s) + 1];
    strcpy(style, s);
}

HasDMA::HasDMA(const HasDMA &hs) {
    style = new char [strlen(hs.style) + 1];
    strcpy(style, hs.style);
}

HasDMA::~HasDMA() {
    delete [] style;
}

HasDMA & HasDMA::operator=(const HasDMA &rs) {
    if (this == &rs)
        return *this;
    BaseDMA::operator=(rs);
    delete [] style;
    style = new char [strlen(rs.style) + 1];
    strcpy(style, rs.style);
    return *this;
}

std::ostream& operator<<(std::ostream &os, const HasDMA &rs) {
    os << (const BaseDMA &) rs;
    os << "Style: " << rs.style << endl;
    return os;
}
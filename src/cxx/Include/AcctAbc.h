//
// Created by wangww on 2020/4/1.
//

#ifndef TM_ACCTABC_H
#define TM_ACCTABC_H


#include <string>
#include <iostream>

// Abstract Base Class
class AcctABC {
private:
    std::string fullName;
    long accNum;
    double balance;

protected:
    struct Formatting{
        std::ios_base::fmtflags flag;
        std::streamsize pr;
    };
    const std::string & FullName() const {
        return fullName;
    }
    long AcctNum() const { return accNum;}
    Formatting SetFormat() const ;
    void Restore(Formatting &f) const ;

public:
    AcctABC(const std::string &s = "Null body",
            long an=-1,
            double bal = 0.0);
    void Deposit(double amt);
    virtual void Withdraw(double amt) = 0;
    double Balance() const { return balance;};
    virtual void ViewAcct() const = 0;
    virtual ~AcctABC();
};

// Brass Account Class
class Brass:public AcctABC{
public:
    Brass(const std::string &s = "Null body",
          long an = -1, double bal = 0.0):AcctABC(s, an, bal){};
    virtual void Withdraw(double amt);
    virtual void ViewAcct() const ;
    virtual ~Brass();
};

class BrassPlus:public AcctABC{
private:
    double maxLoan;
    double rate;
    double owesBank;
public:
    BrassPlus(const std::string &s = "Null body",
              long an = -1,
              double bal = 0.0,
              double m1 = 500,
              double r = 0.10);
    BrassPlus(const Brass & ba, double m1 = 500, double r = 0.1);
    virtual void ViewAcct() const ;
    virtual void Withdraw(double amt);
    void ResetMax(double r){maxLoan = r;};
    void ResetRate(double r){rate = r;};
    void RestOwes(){owesBank = 0;};
};

#endif //TM_ACCTABC_H

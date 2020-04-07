//
// Created by wangww on 2020/4/1.
//

#include <iostream>
#include "include/AcctAbc.h"

using std::cout;
using std::endl;
using std::ios_base;
// Abstract Base Class
AcctABC::AcctABC(const std::string &s,
                 long an,
                 double bal){
    fullName = s;
    accNum = an;
    balance = bal;
}

void AcctABC::Deposit(double amt) {
    if (amt < 0){
        cout << "Negative deposit not allowed; "
        << "deposit is cancelled.\n";
    } else{
        balance++;
    }
}

void AcctABC::Withdraw(double amt) {
    balance = amt;
}


// protected method for formatting
AcctABC::Formatting AcctABC::SetFormat() const {
    // set up ###.## format
    Formatting f;
    f.flag = cout.setf(ios_base::fixed, ios_base::floatfield);
    f.pr = cout.precision(2);
    return f;
}

void AcctABC::Restore(AcctABC::Formatting &f) const {
    cout.setf(f.flag, ios_base::floatfield);
    cout.precision(f.pr);
}

AcctABC::~AcctABC() {}


// Brass 接口实现

Brass::~Brass() {}
void Brass::Withdraw(double amt) {
    if (amt < 0){
        cout << "withdrawal amount must be positive; "
        << "withdrawal canceled.\n";
    } else if (amt <= Balance()){
        AcctABC::Withdraw(amt);
    } else{
        cout << "withdrawal amount of $" << amt
        << " exceeds your balance.\n"
        << "Withdrawal canceled.\n";
    }
}

void Brass::ViewAcct() const {
    Formatting f = SetFormat();
    cout << "Brass client: " << FullName() << endl;
    cout << "Account Number: " << AcctNum() << endl;
    cout << "Balance:$" << Balance() << endl;
    Restore(f);
}

// BrassPlus Method
BrassPlus::BrassPlus(const std::string &s,
        long an, double bal,
        double m1, double r): AcctABC(s, an, bal) {
    maxLoan = m1;
    owesBank = 0.0;
    rate = r;
}


void BrassPlus::ViewAcct() const {
    Formatting f = SetFormat();
    cout << "BrassPlus Client: " << FullName() << endl;
    cout << "Account Number: " << AcctNum() << endl;
    cout << "Balance:$" << Balance() << endl;
    cout << "Maximum load:$" << maxLoan << endl;
    cout << "Owed to bank:$" << owesBank << endl;
    cout.precision(3);
    cout << "Loan Rate: " << 100 * rate << "%\n";
    Restore(f);
}

void BrassPlus::Withdraw(double amt) {
    Formatting f = SetFormat();
    double bal = Balance();
    if(amt <= bal){
        AcctABC::Withdraw(amt);
    } else if (amt <= bal + maxLoan - owesBank){
        double  advance = amt - bal;
        owesBank += advance * (1.0 + rate);
        cout << "Bank advance:$" << advance <<endl;
        cout << "Finance charge: $" << advance * rate << endl;
        Deposit(advance);
        AcctABC::Withdraw(advance);
    } else{
        cout << "Credit Limit exceeded. Transaction cancelled.\n";
        Restore(f);
    }
}


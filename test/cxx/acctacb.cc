//
// Created by wangww on 2020/4/1.
//

#include <iostream>
#include <string>
#include "include/AcctAbc.h"

const int CLIENTS = 4;

int main(){
    using std::cin;
    using std::cout;
    using std::endl;

    AcctABC * p_clientS[CLIENTS];
    std::string temp;
    long tempNum;
    double tempBal;
    char kind;

    for (auto & i : p_clientS) {
        cout << "Enter client`s name: ";
        getline(cin, temp);
        cout << "Enter client`s account number:";
        cin >> tempNum;
        cout << "Enter opening balance :$";
        cin >> tempBal;
        cout << "Enter 1 for Brass Account or 2 for BrassPlus Account:";
        while (cin >> kind && (kind != '1' && kind != '2')){
            cout << "Enter 1 or 2:";
        }
        if (kind == '1')
            i = new Brass(temp, tempNum, tempBal);
        else{
            double tmax, trate;
            cout << "Enter the overdraft limit: $";
            cin >> tmax;
            cout << "Enter the interest rate as a decimal fraction: ";
            cin >> trate;
            i = new BrassPlus(temp, tempNum, tempBal, tmax, trate);
        }
        while (cin.get() != '\n')
            continue;
    }
    cout << endl;
    for (auto & j : p_clientS) {
        j -> ViewAcct();
        cout << endl;
        delete j;
    }
}
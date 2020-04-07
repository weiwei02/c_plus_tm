// 分别使用数组和指针两种不同形式的字符串
// Created by wangww on 2020/3/26.
//
#include <iostream>
#include <cstring>

int  ptrstr(){
    using namespace std;
    char animal[20] = "bear";
    const char *bird = "wren";
    char *ps;

    cout << animal << " and ";
    cout << bird << endl;

    cout << "Enter a kind of animal: ";
    cin >> animal;

    ps = animal;
    cout << ps << "!\n";
    cout << animal << " at " << (int*)animal << endl;
    cout << ps << " at " << (int*)ps << endl;

    ps = new char [strlen(animal) + 1];
    strcpy(ps, animal);

    cout << "After using strcpy(): \n";
    cout << animal << " at " << (int*)animal << endl;
    cout << ps << " at " << (int*)ps << endl;

    delete [] ps;
    return 0;


}
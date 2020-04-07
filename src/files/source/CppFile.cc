// 使用c++标准库对文件操作
// Created by wangww on 2020/3/29.
//
#include <fstream>
#include <iostream>
extern char *path;
using namespace std;
int tCStream(){
    char data[1024], buf2[] = "hello world";
    fstream in;
    ofstream out;
    in.open(path, ios::in | ios::out| ios::app);

    if (!in){
        cout << "open file fail!" << endl;
        return 1;
    }

    in.read(data, 100);
    if (in.eof()){
        // 读取到文件尾
        cout << "完成文件读取" << endl;
        int count = in.gcount();
        char copy[count];
        memcpy(copy, data, count * sizeof(char));
        cout << endl;
    } else if (in.bad()){
        cout << "read file fail";
        return 1;
    } else{
        in.seekg(100, ios::beg);
        in.write(buf2, 12);
    }

    in.close();
    return 0;
}
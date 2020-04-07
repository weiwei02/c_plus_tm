// 测试类，使用dma
// Created by wangww on 2020/4/1.
//

#include <iostream>
#include "include/Dma.h"

int main(){
    using std::cout;
    using std::endl;

    BaseDMA shirt("Protablelly", 8);
    LacksDMA balloon("red", "Blimpo", 4);
    HasDMA map("Mercator", "Buffalo Keys", 5);
    cout << "Displaying BaseDMA object: \n";
    cout << shirt << endl;
    cout << "Displaying LacksDMA object: \n";
    cout << balloon << endl;
    cout << "Displaying HasDMA object: \n";
    cout << map << endl;

    LacksDMA balloon2(balloon);
    cout << "Result of lacksDMA copy:\n";
    cout << balloon2 << endl;

    HasDMA map2;
    map2 = map;
    cout << "Result of HasDMA assignment:\n";
    cout << map2 << endl;
    return 0;
}
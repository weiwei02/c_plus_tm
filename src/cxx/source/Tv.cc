//
// Created by wangww on 2020/4/4.
//

#include "include/Tv.h"
#include <iostream>

bool Tv::volUp() {
    if (volume < MaxVal){
        volume ++;
        return true;
    }
    return false;
}

bool Tv::volDown() {
    if (volume > MinVal){
        volume --;
        return true;
    }
    return false;
}

void Tv::chanUp() {
    if (channel < maxchannel){
        channel ++;
    } else
        channel = 1;
}

void Tv::chanDown() {
    if (channel > 1)
        channel --;
    else
        channel = maxchannel;
}

void Tv::settings() const {
    using std::cout;
    using std::endl;
    cout << "Volume setting = " << volume << endl;
    cout << "Channel setting = " << channel << endl;
    cout << "Mode = "
        << ((mode == Antenna) ? "antenna" : "cable") << endl;
    cout << "Input = "
        << ((input == TV ? "TV" : "DVD")) << endl;

}

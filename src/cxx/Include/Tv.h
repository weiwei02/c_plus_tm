// 友元类可直接访问原类的私有属性
// Created by wangww on 2020/4/4.
// 以电视和遥控器为例，使用友元类

#ifndef TM_TV_H
#define TM_TV_H


class Tv {
public:
    // Remote can access tv private parts
    friend class Remote;
    enum {Off,On};
    enum {MinVal, MaxVal = 20};
    enum {Antenna, Cable};
    enum {TV, DVD};

    Tv(int s = Off, int mc = 125):
    state(s), volume(mc), maxchannel(mc), channel(2),
    mode(Cable), input(TV){};

    void onOff(){
        state = (state == On) ? Off : On;
    }
    bool isOn() const { return state == On;};
    bool volUp();
    bool volDown();
    void chanUp();
    void chanDown();
    void setMode(){
        mode = (mode == Antenna) ? Cable : Antenna;
    }
    void setInput(){
        input = (input == TV) ? DVD : TV;
    }
    // display all setting
    void settings() const ;

private:
    // on or off
    int state;
    //assumed to digitized
    int volume;
    //maximum number of channels
    int maxchannel;
    // current channel setting
    int channel;
    // broadcast or cable
    int mode;
    //TV or DVD
    int input;
};

// 遥控器类
class Remote{
private:
    // control tv or dvd
    int mode;

public:
    Remote(int m = Tv::TV) : mode(m){};

    void onOff(Tv & t){
        t.onOff();
    }
    bool volUp(Tv & t){ t.volUp();};
    bool volDown(Tv & t){ t.volDown();};
    void chanUp(Tv & t) { t.chanUp();};
    void chanDown(Tv & t) { t.chanDown();} ;
    void setMode(Tv & t){
        t.setMode();
    }
    void setInput(Tv & t){
        t.setInput();
    }

    void setChan(Tv & t, int c){
        t.channel = c;
    }
};

#endif //TM_TV_H

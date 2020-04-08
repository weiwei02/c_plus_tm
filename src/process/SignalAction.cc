// sigaction
// Created by wangww on 2020/4/7.
//

#include <csignal>
#include <unistd.h>
#include <iostream>

void oath(int sig){
    std::cout << "OATH: - I GOT A SIGNAL: " << sig << std::endl;
}
int signalAction(){
    struct sigaction act;
    act.__sigaction_u.__sa_handler = oath;
    act.sa_flags = 0;

    sigaction(SIGINT, &act, 0);

    while (1){
        std::cout << "Hello SIGINT \n";
        sleep(1);
    }
}
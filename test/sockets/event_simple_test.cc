#include <iostream>
#include "include/libev.h"

#define SERVER_PORT 49915

int main(){
    simple_server(SERVER_PORT);
    simple_client("127.0.0.1" , SERVER_PORT);
}
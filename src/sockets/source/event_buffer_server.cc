#include <iostream>
#include <cstring>

#include <errno.h>

#include "event.h"
#include "event2/bufferevent.h"

using namespace std;

int tcp_server_init2(int);
int accept_cb2(int fd, short events, void *arg);
int sockert_read_cb2(struct bufferevent *bev, void *arg);
int event_cb2(struct bufferevent *bev, short events, void *arg);

int buffer_server(int port){
    int listener = tcp_server_init2(port);
    if (listener == -1)
    {
        cout << "tcp server init error ";
        return -1; 
    }
    struct event_base *base = event_base_new();

    // 添加客户端请求连接事件
    // struct event * ev_listen = event_new(base, listener, EV_READ| EV_PERSIST, )
    return 0;
}
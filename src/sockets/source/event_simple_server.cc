#include <iostream>
#include <cstring>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <errno.h>

#include "include/event.h"

using namespace std;
int tcp_server_init(int);
void accept_cb(evutil_socket_t, short, void *);

int simple_server(int port){
    
    int lfd = tcp_server_init(port);
    if (lfd <= 0)
    {
        perror("tcp_server_init ");
    }
    
    struct event_base* base = event_base_new();
    
    // 添加客户端请求连接事件
    struct event *ev_listen = event_new(base, lfd, EV_READ | EV_PERSIST, accept_cb, base);

    // 添加事件
    event_add(ev_listen, NULL);
    // 循环直到没有已注册的事件
    event_base_dispatch(base);
    return 0;
}


// 初始化服务端
int tcp_server_init(int port){
    int s_fd = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in server_addr;

    // bind
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    server_addr.sin_port = htons(port);
    bind(s_fd, (struct sockaddr*)&server_addr, sizeof(server_addr));
    if(listen(s_fd, 10) > -1){
        return s_fd;
    }
    return -1;
}


/**
 * 接收客户端新建连接方法
 * 监听接收到新连接后，会回调这个方法
*/
void accept_cb(evutil_socket_t, short, void *){

}
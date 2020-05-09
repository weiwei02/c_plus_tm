#include <iostream>
#include <cstring>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <errno.h>
#include <unistd.h>

#include "event.h"

using namespace std;
int tcp_server_init(int);
void accept_cb(evutil_socket_t, short, void *);
void server_socket_read_cb(evutil_socket_t, short, void *);

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

    // 允许地址被多次绑定，需要声明在 socket 和 bind 之间
    evutil_make_listen_socket_reuseable(s_fd);
    struct sockaddr_in server_addr;
    // bind
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    server_addr.sin_port = htons(port);
    bind(s_fd, (struct sockaddr*)&server_addr, sizeof(server_addr));

    //跨平台统一接口，将套接字设置为非阻塞状态
    evutil_make_socket_nonblocking(s_fd);

    if(listen(s_fd, 10) > -1){
        return s_fd;
    }
    return -1;
}


/**
 * 接收客户端新建连接方法
 * 监听接收到新连接后，会回调这个方法
*/
void accept_cb(evutil_socket_t l_fd, short, void * arg){
    struct sockaddr_in client;
    socklen_t len = sizeof(client);
    evutil_socket_t sockefd = accept(l_fd, (struct sockaddr*)&client, &len);
    evutil_make_socket_nonblocking(sockefd);
    cout << "accept client socket fd " << sockefd << endl;

    struct event_base *base = (event_base *) arg;
    struct event *ev = event_new(NULL, -1, 0, NULL, NULL);
    event_assign(ev, base, l_fd, EV_READ | EV_PERSIST, server_socket_read_cb, (void*) ev);

    event_add(ev, NULL);
}


/**
 * 读socket处理函数
*/
void server_socket_read_cb(evutil_socket_t fd, short, void * arg){
    char msg[4096];
    struct event *ev = (struct event*)arg;
    int len = read(fd, msg, sizeof(msg) - 1);

    if (len <= 0)
    {
        cout << "some error happen when read" << endl;
        event_free(ev);
        close(fd);
        return;
    }
    msg[len] = '\0';
    cout << "receive the client msg: " << msg << endl;
    
    char reply_msg[4096] = "I have received the msg: ";
    strcat(reply_msg + strlen(reply_msg), msg);

    write(fd, reply_msg, strlen(reply_msg));
}
#include <iostream>
#include <cstring>


#include<unistd.h>
#include <errno.h>
#include<sys/socket.h>
#include<netinet/in.h>
#include<arpa/inet.h>

#include "event.h"
#include "event2/bufferevent.h"

using namespace std;

int tcp_connect_server3(const char* server_ip, int port);
void cmd_msg_cb3(int fd,short events,void* args);
void server_msg_cb3(struct bufferevent* bev, void *arg);
void event_cb3(struct bufferevent *bev, short event, void *arg);

int buffer_client(int port){

    int sockfd = tcp_connect_server3("127.0.0.1", port);
    if (sockfd == -1)
    {
        cerr << "tcp connect error " << endl;
        return -1;
    }
    
    cout << "connection to server successful" <<endl;
    struct event_base* base = event_base_new();
    struct bufferevent* bev = bufferevent_socket_new(base, sockfd, BEV_OPT_CLOSE_ON_FREE);

    // 监听终端输入事件
    struct event* ev_cmd = event_new(base, STDIN_FILENO,EV_READ |EV_PERSIST, cmd_msg_cb3, (void*)bev);
    event_add(ev_cmd, NULL);

    bufferevent_setcb(bev, server_msg_cb3, NULL, event_cb3, (void*)ev_cmd);
    bufferevent_enable(bev, EV_READ | EV_PERSIST);

    event_base_dispatch(base);

    cout << "finished" << endl;
    return 0;
}

void cmd_msg_cb3(int fd,short events,void* args){
    char msg[1024];
    int ret = read(fd, msg, sizeof(msg));
    if(ret < 0){
        cerr << "read std input fail ";
        exit(1);
    }
    struct bufferevent* bev = (struct bufferevent*)args;

    bufferevent_write(bev, msg, ret);
}

void server_msg_cb3(struct bufferevent* bev, void *arg){
    char msg[1024];
    size_t len = bufferevent_read(bev, msg, sizeof(msg));
    msg[len] = '\0';

    cout << "recv " << msg << " from server " << endl;
}
void event_cb3(struct bufferevent *bev, short event, void *arg){
    if (event & BEV_EVENT_EOF)
    {
        cerr << "connection closed" << endl;
    }else if (event & BEV_EVENT_ERROR){
        cerr << "some other error" << endl;
    }
    // 自动关闭缓冲区和套接字
    bufferevent_free(bev);

    // socket 已经没有了，这个标准输入事件也没有继续监听的必要了。
    struct event* ev = (struct event*)arg;
    event_free(ev);
}

int tcp_connect_server3(const char* server_ip, int port){
    struct sockaddr_in server_addr;
    memset(&server_addr, 0 , sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(port);
    int status = inet_aton(server_ip, &server_addr.sin_addr);

    if(status == 0){
        // the server ip in not valid value
        errno = EINVAL;
        return -1;
    }
    int sockfd = socket(PF_INET, SOCK_STREAM, 0);
    if (sockfd == -1)
    {
        return -1;
    }

    status = connect(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr));
    if (status == -1)
    {
        // the close may be error
        int save_errno = errno;
        close(sockfd);
        errno = save_errno;
        return -1;
    }

    evutil_make_socket_nonblocking(sockfd);
    return sockfd;
    
}
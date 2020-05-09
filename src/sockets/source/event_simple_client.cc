#include <iostream>
#include <cstring>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <errno.h>
#include <unistd.h>


#include "event.h"

using namespace std;

int tcp_connect_server(const char*, int);

void cmd_msg_cb(int, short, void*);
void client_socket_read_cb(int, short, void*);

// 客户端
int simple_client(char* ip, int port){
    int sockfd = tcp_connect_server(ip, port);
    if (sockfd == -1)
    {
        perror("tcp connect error ");
        return -1;
    }

    cout << "connect to server successful " << endl;    
    struct event_base *base = event_base_new();
    struct event *ev_sockfd = event_new(base, sockfd, EV_READ | EV_PERSIST, client_socket_read_cb, NULL);
    event_add(ev_sockfd, NULL);
    
    struct event* ev_cmd = event_new(base, STDIN_FILENO, EV_READ | EV_PERSIST, cmd_msg_cb, (void*)&sockfd);
    event_add(ev_cmd, NULL);
    event_base_dispatch(base);

    cout << "finished" << endl;
    return 0;
}

// 消息读取回调函数
void cmd_msg_cb(int fd, short events, void *arg){
    char msg[1024];

    int ret = read(fd, msg, sizeof(msg));
    if(ret <= 0){
        perror("read fail ");
        exit(1);
    }

    int sockfd = *((int *) arg);

    // 把终端的消息发送给服务器
    write(sockfd, msg, ret);
}

void client_socket_read_cb(int fd, short events, void *arg){
    char msg[1024];

    int len = read(fd, msg, sizeof(msg) - 1);
    if (len <= 0)
    {
        perror("read fail ");
        exit(1);
    }
    msg[len] = '\0';
    cout << "recv " << msg << " from server" << endl;
}

typedef struct sockaddr SA;
int tcp_connect_server(const char* server_ip, int port){
    int sockfd, status, save_errno;
    struct sockaddr_in server_addr;

    memset(&server_addr, 0 , sizeof(server_addr));

    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(port);
    status = inet_aton(server_ip, &server_addr.sin_addr);
    if (status == 0)
    { // the server ip is not valid value
        errno = EINVAL;
        return -1;
    }

    sockfd = socket(PF_INET, SOCK_STREAM, 0);
    if (sockfd == -1)
    {
        return -1;
    }

    status = connect(sockfd, (SA*)&server_addr, sizeof(server_addr));
    if(status == -1){
        save_errno = errno;
        close(sockfd);
        // the close may error
        errno = save_errno;
        return -1;
    }
    evutil_make_socket_nonblocking(sockfd);
    return sockfd;
}
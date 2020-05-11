
#include<netinet/in.h>  
#include<sys/socket.h>  
#include<unistd.h>  
  
#include<iostream>  
#include<cstring>  
#include<thread>  
  
#include "event.h"  
#include "listener.h"
#include "event2/bufferevent.h"  
#include "thread.h"  

using namespace std;

void listener_cb4(evconnlistener *listener, evutil_socket_t fd, 
    struct sockaddr *sock, int socklen, void *arg);

void socket_read_cb4(struct bufferevent *bev, void *arg);
void socket_event_cb4(struct bufferevent *bev,short events, void *arg);

int buffer_event_listener(int port){
    struct sockaddr_in sin;
    memset(&sin, 0, sizeof(struct sockaddr_in));
    sin.sin_family = AF_INET;
    sin.sin_port = htons(port);

    event_base *base = event_base_new();
    evconnlistener *listener = evconnlistener_new_bind(base, listener_cb4, base, 
        LEV_OPT_REUSEABLE | LEV_OPT_CLOSE_ON_FREE, 10, 
        (struct sockaddr*)&sin, sizeof(struct sockaddr_in));
    
    event_base_dispatch(base);

    evconnlistener_free(listener);
    event_base_free(base);
    return 0;
}


//一个新客户端连接上服务器了  
//当此函数被调用时，libevent已经帮我们accept了这个客户端。该客户端的
//文件描述符为fd
void listener_cb4(evconnlistener *listener, evutil_socket_t fd, 
    struct sockaddr *sock, int socklen, void *arg){
        cout << "accept aclient " << fd << endl;

        event_base *base = (event_base*)arg;

        // 为客户端分配一个event_buffer
        bufferevent *bev = bufferevent_socket_new(base, fd, BEV_OPT_CLOSE_ON_FREE); 
        bufferevent_setcb(bev, socket_read_cb4, NULL, socket_event_cb4, NULL);
        bufferevent_enable(bev, EV_READ|EV_PERSIST);
    }

void socket_read_cb4(struct bufferevent *bev, void *arg){
     char msg[4096];  
  
    size_t len = bufferevent_read(bev, msg, sizeof(msg)-1 );  
  
    msg[len] = '\0';  
    cout << "server read the data " <<  msg << endl;
  
    char reply[] = "I has read your data";  
    bufferevent_write(bev, reply, strlen(reply) );
}

void socket_event_cb4(struct bufferevent *bev,short events, void *arg){
    if (events & BEV_EVENT_EOF)  
        cerr << "connection closed\n";
    else if (events & BEV_EVENT_ERROR)  
        cerr << ("some other error\n");  
  
    //这将自动close套接字和free读写缓冲区  
    bufferevent_free(bev);
}
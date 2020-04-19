#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include <iostream>

int tcp_server(int port){
    // 服务端和客户端套接字地址
    struct sockaddr_in s_addr, c_addr;

    // 创建套接字
    int s_fd = socket(AF_INET, SOCK_STREAM, 0);
    // 定义服务器套接字中的域
    s_addr.sin_family = AF_INET;
    // 定义套接字地址
    s_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    s_addr.sin_port = port;
    
    // 绑定套接字和端口号
    if (bind(s_fd, (struct sockaddr *)&s_addr, sizeof(s_addr)) == -1) return -1;
    // 监听状态，守候进程
    if(listen(s_fd, 10) == -1){
        std::cout << "监听服务端socket错误 \n";
        return -1;
    }

    char * send_buf = "thanks";
    char buf[256];
    while (1)
    {
        std::cout << "please wait a moment " << std::endl;

        int c_len = sizeof(c_addr);
        int c_fd = accept(s_fd,(sockaddr *) &c_addr,(socklen_t *) &c_len);
        if (c_fd == -1)
        {
            std::cout  << "接收客户端连接错误\n";
            return -1; 
        }
        // 接收消息
        ssize_t len = recv(c_fd, buf, 256, 0);
        buf[len] = '\0';
        std::cout << "receive message : " << buf << std::endl;

        // 发送消息
        send(c_fd, send_buf, sizeof(send_buf), 0);
        close(c_fd);
    }
}
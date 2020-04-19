#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include <iostream>

int tcp_client(int port){
    int socket_fd = socket(AF_INET, SOCK_STREAM, 0);
    
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_ANY);
    addr.sin_port = port;
    int new_socket_fd = connect(socket_fd, (sockaddr *)&addr, sizeof(addr));
    if (new_socket_fd == -1)
    {
        std::cerr << "连接失败\n";
        return -1;
    }
    char *buf = "come on!";
    send(socket_fd, buf, sizeof(buf), 0);
    sleep(10);
    
    char read_buf[40];
    ssize_t len = recv(socket_fd, read_buf, 256, 0);
    read_buf[len] = '\0';

    std::cout << "receive message: " << read_buf << std::endl;
    close(socket_fd);
    return 0;
}
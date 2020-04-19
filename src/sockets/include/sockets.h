#ifndef TM_SOCKETS_H
#define TM_SOCKETS_H
// 建立tcp服务端
int tcp_server(int port);
int tcp_client(int port);

/**
 * int      select(int, fd_set * read_fds, fd_set * write_fds,
    fd_set * error_fds, struct timeval * timeout)
    
 * select调用用于测试文件描述符集合中，是否有一个文件描述符已处于可读状态或可写状态或错误状态，它将阻塞以等待某个文件描述符进入上述这些状态。
参数nfds指定需要测试的文件描述符数目，测试的描述符范围从0到nfds-1。3个描述符集合都可以被设为空指针，这表示不执行相应的测试。
select函数会在发生以下情况时返回：readfds集合中有描述符可读、writefds集合中有描述符可写或errorfds集合中有描述符遇到错误条件。
如果这3种情况都没有发生，select将在timeout指定的超时时间经过后返回。如果timeout参数是一个空指针并且套接字上也没有任何活动，这
个调用将一直阻塞下去。当select返回时，描述符集合将被修改以指示哪些描述符正处于可读、可写或有错误的状态。我们可以用FD_ISSET对描
述符进行测试，来找出需要注意的描述符。
你可以修改timeout值来表明剩余的超时时间，但这并不是在X/Open规范中定义的行为。如果select是因为超时而返回的话，所有描述符集合都
将被清空。
s
elect调用返回状态发生变化的描述符总数。失败时它将返回-1并设置errno来描述错误。可能出现的错误有：EBADF（无效的描述符）、EINTR
（因中断而返回）、EINVAL（nfds或timeout取值错误）。

*/
int select_from_stdin();
#endif
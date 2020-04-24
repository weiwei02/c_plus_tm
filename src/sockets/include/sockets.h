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
select调用返回状态发生变化的描述符总数。失败时它将返回-1并设置errno来描述错误。可能出现的错误有：EBADF（无效的描述符）、EINTR
（因中断而返回）、EINVAL（nfds或timeout取值错误）。

*/
int select_from_stdin();

/**
 * poll() 执行的任务和select()很相似，两者之间的主要区别是如何制定待检查的文件描述符。器函数原型如下：
 * 
 * #include <poll.h>
 * int poll(struct pollfd fds[], nfds_t nfds, int timeout)
 * 
 * 返回已经准备完毕的文件描述符数。超时返回0，错误返回 -1
 * 参数 fds列出了需要poll来检查的文件描述符。其定义如下：
 * struct pollfd{
 *  int fd;
 *  // Requested events bit mask
 *  short events;
 *  // Returned events bit mask
 *  shot revents;
 * }
 * 
 * 参数nfds 指定了 fds的元素个数。数据类型 nfds_t 实际为无符号整形
 * 
 * timeout 参数决定了poll的阻塞行为：
 * -1： 一直阻塞
 * 0：不会阻塞，只检查一次
 * >0 : 至多阻塞 n 毫秒。
*/
int poll_pipes(int pipes_num,int wirte_num);


/**
 * epoll API 是linux专有的，在2.6版新增
 * 同I/O多路复用和信号驱动I/O一样，epoll可以检查多个文件描述符上I/O就绪状态。epoll API的主要优点如下：
 * * 当检查大量的文件描述符时，epoll函数的性能延展性比select和poll好很多
 * * epoll API既支持水平触发也支持边缘触发。select和poll只支持水平触发，而信号驱动IO只支持边缘触发
 * 在性能表现上，epoll同信号驱动I/O相似。但是epoll有一些胜过信号驱动I/O的优点：
 * * 可以避免复杂的信号处理（比如信号队列溢出时的处理）
 * * 灵活性高，可以指定我们希望检查的事件类型。
 * 
 * epoll API的核心数据结构被称作epoll实例，它和一个打开的文件描述符相关联。这个文件描述符不是用来做I/O
 * 操作的，它是内核数据结构的句柄，这些内核数据结构实现了两个目的。
 * - 记录了进程中声明的感兴趣的文件描述符列表 -- interest list
 * - 维护处于I/O就绪状态的文件描述符列表 -- ready list。
 * 
 * 对于epoll检查的每一个文件描述符，可以指定一个位掩码标识我们感兴趣的事件。epoll API由以下三个系统调用
 * 组成:
 * 
 * #include <sys/epoll.h>
 * 
 * 1. 创建一个epoll实例，返回代表该实例的文件描述符。
 * 参数size标识想要告诉内核如何为内部数据结构划分初始大小，并不是上限。
 * 
 * int epoll_create(int size)
 * 
 * 2. 修改epoll感兴趣的列表
 * 参数fd指明了要修改兴趣列表中哪一个文件描述符的设定。这里的fd不能是普通文件或目录的文件描述符，会出现
 * EPERM 错误。
 * 参数op用来指定需要执行的操作，它有以下取值：
 * EPOLL_CTL_ADD 如果试图添加已存在的文件描述符，会产生 EEXIT 错误
 * EPOLL_CTL_MOD  如果试图修改不在兴趣列表中的文件描述符，会产生 ENOENT 错误
 * EPOLL_CTL_DEL
 * 
 * 参数ev是指向 epoll_event 的指针，结构体定义如下：
 * struct epoll_event{
 *  uint32_t events; // epoll events(bit mask)
 *  epoll_data_t data; // user data
 * }
 * 
 * typedef union epoll_data{
 *      void *ptr; // pointer to user define data
 *      int fd;
 *      uint32_t u32;
 *      uint64_t u64;
 * } epoll_data_t;
 * data字段是一个数据联合体，当描述符fd成为就绪状态
 * 
 * int epoll_ctl(int epfd, int op, int fd, struct epoll_event *ev);
 * 
 * 3. 事件等待
 * 系统调用 epoll_wait 返回epoll实例中处于就绪态的文件描述符信息。
 * 参数evlist指向的结构体数组中返回的有关就绪状态文件描述符的信息。数组evlist的空间由调用者负责
 * 申请，所包含的元素个数在maxevents中指定。
 * 调用成功后，epoll_wait返回数组evlist中就绪的文件个数，如果没有就绪文件返回0，错误返回-1
 * 
 * int epoll_wait(int epfd, struct epoll_event *evlist, int maxevents, int timeout);
*/
int epoll_input();
#endif
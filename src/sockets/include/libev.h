/**
 * 简单libevent使用
 * 
 * 
 使用 libevent 函数之前需要分配一个或者多个 event_base 结构体。每个 event_base 结构 体持有一个事件集合,可以检测以确定哪个事件是激活的。
如果设置 event_base 使用锁,则可以安全地在多个线程中访问它 。然而,其事件循环只能 运行在一个线程中。如果需要用多个线程检测 IO,则需要为每个线程使用一个 event_base。
每个 event_base 都有一种用于检测哪种事件已经就绪的 “方法”,或者说后端。可以识别的方法有:
    select
    poll
    epoll
    kqueue
    devpoll
    evport
    win32

1 创建默认的event_base
event_base_new()函数分配并且返回一个新的具有默认设置的 event_base。函数会检测环境变量,返回一个到 event_base 的指针。如果发生错误,则返回 NULL。选择各种方法时,函数会选择 OS 支持的最快方法。
struct event_base *event_base_new(void);


 * 1.  使用 event_new()接口创建事件。
#define EV_TIMEOUT      0x01
#define EV_READ         0x02
#define EV_WRITE        0x04
#define EV_SIGNAL       0x08
#define EV_PERSIST      0x10
#define EV_ET           0x20

typedef void (*event_callback_fn)(evutil_socket_t, short, void *);

struct event *event_new(struct event_base *base, evutil_socket_t fd,
    short what, event_callback_fn cb,
    void *arg);

void event_free(struct event *event);
event_new()试图分配和构造一个用于 base 的新的事件。 what 参数是上述标志的集合。
如果 fd 非负,则它是将被观察其读写事件的文件。
事件被激活时, libevent 将调用 cb 函数,
传递这些参数:文件描述符 fd,表示所有被触发事件的位字段 ,以及构造事件时的 arg 参数。
发生内部错误,或者传入无效参数时, event_new()将返回 NULL。
所有新创建的事件都处于已初始化和非未决状态 ,调用 event_add()可以使其成为未决的。
要释放事件,调用 event_free()。对未决或者激活状态的事件调用 event_free()是安全 的:在释放事件之前,函数将会使事件成为非激活和非未决的。

2. 事件标识 

EV_TIMEOUT
这个标志表示某超时时间流逝后事件成为激活的。构造事件的时候,EV_TIMEOUT 标志是 被忽略的:可以在添加事件的时候设置超时 ,也可以不设置。超时发生时,回调函数的 what 参数将带有这个标志。
EV_READ
表示指定的文件描述符已经就绪,可以读取的时候,事件将成为激活的。
EV_WRITE
表示指定的文件描述符已经就绪,可以写入的时候,事件将成为激活的。
EV_SIGNAL 用于实现信号检测,请看下面的 “构造信号事件”节。
EV_PERSIST 表示事件是“持久的”,请看下面的“关于事件持久性”节。
EV_ET
表示如果底层的 event_base 后端支持边沿触发事件,则事件应该是边沿触发的。这个标志 影响 EV_READ 和 EV_WRITE 的语义。
从2.0.1-alpha 版本开始,可以有任意多个事件因为同样的条件而未决。比如说,可以有两 个事件因为某个给定的 fd 已经就绪,可以读取而成为激活的。这种情况下,多个事件回调 被执行的次序是不确定的。

3 关于事件持久性
默认情况下,每当未决事件成为激活的(因为 fd 已经准备好读取或者写入,或者因为超时), 事件将在其回调被执行前成为非未决的。如果想让事件再次成为未决的 ,可以在回调函数中 再次对其调用 event_add()。
然而,如果设置了 EV_PERSIST 标志,事件就是持久的。这意味着即使其回调被激活 ,事件还是会保持为未决状态 。如果想在回调中让事件成为非未决的 ,可以对其调用 event_del ()。
每次执行事件回调的时候,持久事件的超时值会被复位。因此,如果具有 EV_READ|EV_PERSIST 标志,以及5秒的超时值,则事件将在以下情况下成为激活的:
套接字已经准备好被读取的时候
从最后一次成为激活的开始,已经逝去 5秒

4 信号事件
libevent 也可以监测 POSIX 风格的信号。要构造信号处理器,使用:
#define evsignal_new(base, signum, cb, arg) \
    event_new(base, signum, EV_SIGNAL|EV_PERSIST, cb, arg)
除了提供一个信号编号代替文件描述符之外,各个参数与 event_new()相同。
*/
int simple_server(int port);
int simple_client(char* , int port);


/**
 * 数据缓冲Bufferevent
很多时候,除了响应事件之外,应用还希望做一定的数据缓冲。比如说,写入数据的时候 ,通常的运行模式是:
决定要向连接写入一些数据,把数据放入到缓冲区中
等待连接可以写入
写入尽量多的数据
记住写入了多少数据,如果还有更多数据要写入,等待连接再次可以写入
这种缓冲 IO 模式很通用,libevent 为此提供了一种通用机制,即bufferevent。
bufferevent 由一个底层的传输端口(如套接字 ),一个读取缓冲区和一个写入缓冲区组成。与通常的事件在底层传输端口已经就绪,
可以读取或者写入的时候执行回调不同的是,bufferevent 在读取或者写入了足够量的数据之后调用用户提供的回调。
有多种共享公用接口的 bufferevent 类型,编写本文时已存在以下类型:
基于套接字的 bufferevent:使用 event_*接口作为后端,通过底层流式套接字发送或者接收数据的 bufferevent
异步 IO bufferevent:使用 Windows IOCP 接口,通过底层流式套接字发送或者接收数据的 bufferevent(仅用于 Windows,试验中)
过滤型 bufferevent:将数据传输到底层 bufferevent 对象之前,处理输入或者输出数据的 bufferevent:比如说,为了压缩或者转换数据。
成对的 bufferevent:相互传输数据的两个 bufferevent。
注意:截止2.0.2-alpha 版,这里列出的 bufferevent 接口还没有完全正交于所有 的 bufferevent 类型。也就是说,下面将要介绍的接口
不是都能用于所有bufferevent 类型。libevent 开发 者在未来版本中将修正这个问题。
也请注意 :当前 bufferevent 只能用于像 TCP 这样的面向流的协议,将来才可能会支持 像 UDP 这样的面向数据报的协议。
bufferevent和evbuffer
每个 bufferevent 都有一个输入缓冲区和一个输出缓冲区 ,它们的类型都是“struct evbuffer”。 有数据要写入到 bufferevent 时,添
加数据到输出缓冲区 ;bufferevent 中有数据供读取的时候,从输入缓冲区抽取(drain)数据。 evbuffer 接口支持很多种操作,后面的章节将讨论这些操作。

 回调和水位
每个 bufferevent 有两个数据相关的回调:一个读取回调和一个写入回调。
默认情况下,从底层传输端口读取了任意量的数据之后会调用读取回调 ;
输出缓冲区中足够量的数据被清空到底层传输端口后写入回调会被调用。
通过调整 bufferevent 的读取和写入 “水位 (watermarks )”可以覆盖这些函数的默认行为。
每个 bufferevent 有四个水位:
读取低水位 :读取操作使得输入缓冲区的数据量在此级别或者更高时 ,读取回调将被调用。默认
值为 0,所以每个读取操作都会导致读取回调被调用。
读取高水位 :输入缓冲区中的数据量达到此级别后, bufferevent 将停止读取,直到输入缓冲
区中足够量的数据被抽取 ,使得数据量低于此级别 。默认值是无限 ,所以永远不会因为输入缓
冲区的大小而停止读取。
写入低水位 :写入操作使得输出缓冲区的数据量达到或者低于此级别时 ,写入回调将被调用。
默认值是 0,所以只有输出缓冲区空的时候才会调用写入回调。
写入高水位 :bufferevent 没有直接使用这个水位。它在 bufferevent 用作另外一 
个 bufferevent 的底层传输端口时有特殊意义。请看后面关于过滤型 bufferevent 的介绍。

数据封装evBuffer
libevent 的 evbuffer 实现了为向后面添加数据和从前面移除数据而优化的字节队列。
evbuffer 用于处理缓冲网络 IO 的“缓冲”部分。它不提供调度 IO 或者当 IO 就绪时触发 IO 的 
功能:这是 bufferevent 的工作

链接监听器evconnlistener
evconnlistener 机制提供了监听和接受 TCP 连接的方法。

 * 
*/
int buffer_server(int port);
int buffer_client(int port);
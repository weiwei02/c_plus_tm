#include <sys/types.h>
#include <sys/time.h>
#include <sys/ioctl.h>
// #include <fcntl.h>
#include <unistd.h>
#include <iostream>

/**
 * “下面这个程序select.c演示了select函数的使用方法。我们稍后还会看到一个更复杂的例子。
 * 这个程序读取键盘（即标准输入——文件描述符为0），超时时间设为2.5秒。它只有在输入就绪时
 * 才读取键盘。它可以很容易地通过添加其他描述符（如串行线、管道、套接字等）进行扩展，
 * 具体做法取决于应用程序的需要。

 * 
*/
int select_from_stdin(){
    char buffer[128];
    fd_set inputs, testfds;
    struct timeval timeout;

/*FD_ZERO用于将fd_set初始化为空集合，FD_SET和FD_CLR分别用于在集合中设置和清除由参数fd传递
的文件描述符。如果FD_ISSET宏中由参数fd指向的文件描述符是由参数fdset指向的fd_set集合中的一个
元素，FD_ISSET将返回非零值。fd_set结构中可以容纳的文件描述符的最大数目由常量FD_SETSIZE指定。*/
    FD_ZERO(&inputs);
    FD_SET(0, &inputs);

    // “在标准输入stdin上最多等待输入2.5秒”
    while (1)
    {
        testfds = inputs;
        timeout.tv_sec = 2;
        timeout.tv_usec = 500000;
        int result = select(FD_SETSIZE, &testfds, (fd_set *) NULL, (fd_set *) NULL, &timeout);

        switch (result)
        {
        case 0:
            std::cerr << "timeout \n" ; 
            break;
        case -1:
            std::cerr << "select error \n" ; 
            return -1;
        default:
        if (FD_ISSET(0, &testfds))
        {
            int nread;
            // 获取可读字符数
            ioctl(0, FIONREAD,&nread);
            if(nread == 0){
                std::cout << "keyboard is done \n";
                return 0;
            }
            nread = read(0, buffer, nread);
            buffer[nread] = 0;
            std:: cout << "read " << nread << " chars : " << buffer << std::endl;
        }
        
            break;
        }
    }
    
}
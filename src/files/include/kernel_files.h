// 内核文件操作
// Created by wangww on 2020/4/7.
//

#ifndef C_PLUS_TM_KERNEL_FILES_H
#define C_PLUS_TM_KERNEL_FILES_H

/**
 * 在文件不同的区域上加上各种锁
 *
 * “fcnt1对一个打开的文件描述符进行操作，并能根据command参数的设置完成不同的任务。它为我们提供了3个用于文件锁定的命令选项：
□　F_GETLK”
□　“F_SETLK
□　F_SETLKW
当使用这些命令选项时，fcnt1的第三个参数必须是一个指向flock结构的指针，所以实际的函数原型应为：

flock（文件锁）结构依赖具体的实现，但它至少包含下述成员：
□　short l_type
□　short l_whence
□　off_t l_start
□　off_t l_len
□　pid_t l_pid”

 “l_whence、l_start和l_len成员定义了文件中的一个区域，即一个连续的字节集合。
 l_whence的取值必须是SEEK_SET、SEEK_CUR、SEEK_END（在头文件unistd.h中定义）中的一个。
 它们分别对应于文件头、当前位置和文件尾。l_whence定义了l_start的相对偏移值，其中，l_start是该区域的第一个字节。
 l_whence通常被设为SEEK_SET，这时l_start就从文件的开始计算。l_len参数定义了该区域的字节数。
l_pid参数用来记录持有锁的进程
“F_GETLK的介绍。
文件中的每个字节在任一时刻只能拥有一种类型的锁：共享锁、独占锁或解锁。

1．F_GETLK命令
第一个命令是F_GETLK。它用于获取fildes（第一个参数）打开的文件的锁信息。
 它不会尝试去锁定文件。调用进程把自己想创建的锁类型信息传递给fcnt1，使用F_GETLK命令的fcnt1就会返回将会阻止获取锁的任何信息。
如果F_GETLK调用成功（例如，它返回一个非-1的值），调用程序就必须检查flock结构的内容来判断其是否被修改过。
 因为l_pid的值被设置成持有锁的进程（如果有的话）的标识符，所以通过检查这个字段就可以很方便地判断出flock结构是否被修改过。

2．F_SETLK命令
这个命令试图对fildes指向的文件的某个区域加锁或解锁。
与F_GETLK一样，要加锁的区域由flock结构中的l_start、l_whence和l_len的值定义。
 如果加锁成功，fcnt1将返回一个非-1的值；如果失败，则返回-1。这个函数总是立刻返回。

3．F_SETLKW命令
F_SETLKW命令与上面介绍的F_SETLK命令作用相同，但在无法获取锁时，这个调用将等待直到可以为止。
 一旦这个调用开始等待，只有在可以获取锁或收到一个信号时它才会返回。

 程序对某个文件拥有的所有锁都将在相应的文件描述符被关闭时自动清除。在程序结束时也会自动清除各种锁.

 unistd.h 还提供了 lockf 函数，其原型如下：
  int flock(int filded,int function, off_t size_to_lock)

 function参数的取值如下所示。
□　F_ULOCK:解锁。
□　F_LOCK:设置独占锁。
□　F_TLOCK:测试并设置独占锁。
□　F_TEST:测试其他进程设置的锁。
size_to_lock参数是操作的字节数，它从文件的当前偏移值开始计算。

摘录来自: 马修(Neil Matthew). “Linux程序设计(第4版) (图灵程序设计丛书•Linux/UNIX系列)。” Apple Books.
 * */
int lock_file_area();
#endif //C_PLUS_TM_KERNEL_FILES_H

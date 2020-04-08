// 进程控制学习
// Created by wangww on 2020/3/8.
//

#ifndef TM_PROCESS_H
#define TM_PROCESS_H

#endif //TM_PROCESS_H

struct name;
struct name{
    char *name;
};

/**
 * 1. fork() 函数的功能是创建一个新进程，新进程为当前进程为当前进程，那么当前的进程就被称为父进程。
 * 在一个函数中，可以通过fork() 函数的返回值判断是否是在子进程中，还是在父进程中。fork()函数的调用形式为：
 *
 * pid_t fork(void)
 *
 * 使用fork函数需要引用 <sys/types.h>和<unistd.h>头文件，该函数的返回值为 pid_t ，表示一个非负整数。
 * 若程序运行在父进程中，函数返回的pid为子进程的进程号；若运行在子进程中，返回的pid的为0；
 * 如果调用 fork() 函数创建子进程失败，那么就会返回-1，并且提示错误信息。错误信息有以下两种形式：
 * EAGAIN: 标识fork()函数没有足够的内存用于父进程的分页表和进程数据结构。
 * ENOMEM: 标识 fork() 函数分配必要的内核数据结构时，内存不足。
 * */
void tfork();


/**
 * vfork() 函数与 fork() 函数相同，都是系统调用函数，两者之间的区别是在创建子进程时 fork() 函数会复制父进程所有的资源，包括进程环境、内存资源等。
 * 而 vfork() 函数在创建子进程时不会复制父进程的所有资源，父子进程共享地址空间。这样，在子进程中对虚拟内存空间中变量的修改，实际上是在修改父进程
 * 虚拟内存空间中的值
 *
 * 在使用 vfork() 函数时，父进程会被阻塞，需要在子进程中调用 _exit() 函数退出子进程，不能使用exit()函数退出
 * */
void tvfork();

/**
 * exec 函数族
 *
 * 通过调用 fork() 函数和vfork()函数创建子进程，子进程和父进程执行的代码是相同的。但是通常创建了一个子进程后，目的是为了和父进程执行不同的操作
 * 实现不同的功能。因此，linux系统提供了一个exec()函数族，用于创建和修改子进程。调用exec()函数时，子进程中的代码段，数据段和堆栈段都被替换。
 * 由于调用exec()函数并没有创建新的进程，因此修改后的子进程id并没有改变。
 *
 * exec 函数由以exec开头的6个函数组成，定义形式分别如下：
 *
 * int execl(const char *path,const char *arg,...)
 * int execlp(const char *path,const char *arg,...)
 * int execle(const char *path,const char *arg,...,char *const envp[])
 * int execv(const char *path,char *const envp[])
 * int execve(const char *path,const char *argv[],char *const envp[])
 * int execvp(const char *path,const char *argv[])
 *
 * 这些函数都定义在系统函数库中，调用成功时没有返回值，调用失败时会返回-1.在使用前需要引用 <sys/types.h>和<unistd.h>，并且必须在预定义时定义一个外部全局变量，例如：
 *
 * extern char **environ
 *
 * 上面定义的变量时一个指向linux全局变量的指针。定义了这个变量后，就可以在当前工作目录中执行系统程序。
 *
 * 这几个exec函数非常相似，但却可以通过其拼写规律区分每个方法的作用。
 * 1） 函数中带有p
 * 字符p是path的首字符，代表文件的绝对路径（或者相对路径）。当函数名中带有字符p时，函数的参数就可以不用写出文件的相对路径，只写出文件名即可，
 * 因为函数会自动搜索系统的path路径。
 * 2）函数名带有字符l
 * 字符l是list的首字母，表示需要将新程序的每一个命令行参数都当做一个参数传递给它，参数的个数是可变的，并且最后要输入一个 NULL 参数，
 * 表示参数输入结束。
 * 3）带有字符v
 * 字符v是vector的首字母，表示该类函数支持使用参数数组，数组中的最后一个指针也要输入 NULL 参数，作为结束标志。
 * 4） 函数名以e结尾
 * e是environment的首字母，该类函数表示可以将一份新的环境变量传递给它。
 *
 * exec函数族里只有 execve()函数时系统调用，其它函数都最终调用了 execve()
 * */

/**
 * 进程等待是为了同步父进程和子进程，通常需要调用wait()函数使父进程等待子进程的结束。如果父进程没有等待子进程结束，子进程就会进入 zombie 状态
 *
 * linux 提供的等待函数原型如下：
 * #include <sys/types.h>
 * #include <sys/wait.h>
 * pid_t wait(int *status)
 * pid_t wait_pid(pid_t pid, int *status, int options)
 * int waitid(idtype_t idtype, id_t id, siginfo_t *infop, int options)
 *
 * 在linux命令行输入 man 2 wait 可以查看wait函数的信息
 *wait函数的工作流程是，首先判断子进程是否存在，如果不存在，函数直接退出，并且提示相关错误信息 ECHILD。
 * 如果有子进程，父进程会被挂起，直到子进程结束，并返回结束时状态和最后结束的子进程PID。
 * 在存在子进程，退出进程时的状态有以下两种：
 * 1） 进程正常结束
 * 函数会返回退出的子进程PID和status，status就是子进程退出码
 * 2）信号引起的子进程结束
 * wait()函数系统调用中会发起信号给子进程，可能会导致子进程异常结束。
 * 若发送的信号被子进程捕获，就会起到终止子进程的作用。
 * 若发送的信号没有被子进程捕捉到，则会使子进程非正常结束。此时status值为接收到的信号值，存放在最后一个字节中。
 *
 * wait_pid() 函数的options参数的取值及其意义如下：
 * 1. WNOHANG : 该参数表示没有子进程退出就立即返回
 * 2. WUNTRACED : 该参数表示若发现子进程处于僵尸状态，但未报告状态，则立即返回。
 * */
 void twait();


 /**
  * “sigaction结构定义在文件signal.h中，它的作用是定义在接收到参数sig指定的信号后应该采取的行动。”

“sigaction函数设置与信号sig关联的动作。如果oact不是空指针，sigaction将把原先对该信号的动作写到它指向的位置。
  如果act是空指针，则sigaction函数就不需要再做其他设置了，否则将在该参数中设置对指定信号的动作。”

“在参数act指向的sigaction结构中，sa_handler是一个函数指针，它指向接收到信号sig时将被调用的信号处理函数。
  它相当于前面见到的传递给函数signal的参数func。我们可以将sa_handler字段设置为特殊值SIG_IGN和SIG_DFL，
  它们分别表示信号将被忽略或把对该信号的处理方式恢复为默认动作。
sa_mask成员指定了一个信号集，在调用sa_handler所指向的信号处理函数之前，该信号集将被加入到进程的信号屏蔽字中。
  这是一组将被阻塞且不会传递给该进程的信号。设置信号屏蔽字可以防止前面看到的信号在它的处理函数还未运行结束时就被接收到的情况。
  使用sa_mask字段可以消除这一竞态条件。”

摘录来自: 马修(Neil Matthew). “Linux程序设计(第4版) (图灵程序设计丛书•Linux/UNIX系列)。” Apple Books.
  * */
 void signalAction();
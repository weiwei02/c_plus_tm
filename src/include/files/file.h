//
// Created by wangww on 2020/3/23.
//

#ifndef TM_FILE_H
#define TM_FILE_H

/**
 * 文件索引(inode)信息
 * linux 的每一个文件都是由目录和索引节点构成的，实际的文件内存存放在磁盘中，由索引找到实际的磁盘位置
 *
 * struct stat { /* when _DARWIN_FEATURE_64_BIT_INODE is NOT defined
    dev_t    st_dev;    // device inode resides on
    ino_t    st_ino;    // inode's number
    mode_t   st_mode;   // inode protection mode
    nlink_t  st_nlink;  // number of hard links to the file
    uid_t    st_uid;    // user-id of owner
    gid_t    st_gid;     //group-id of owner
    dev_t    st_rdev;   // device type, for special file inode
    struct timespec st_atimespec;   time of last access
    struct timespec st_mtimespec;   time of last data modification
    struct timespec st_ctimespec;   time of last data modification
 * */

/**
 * 改变文件权限
 * int chmod(const char *path,mode_t mode)
 * int fchmod(int fileds,mode_t mode)
 *
 *
 * 这个测试为目标文件增加可执行权限7
 * */
int t_chmod(const char *path);

// 测试内核的文件接口读写与操作文件
int tKernelFile();

// 测试c标准库io操作
int tCStdIO();

// 测试c++标准库io读写操作
int tCStream();
#endif //TM_FILE_H

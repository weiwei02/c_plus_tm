// 用在文件不同区域的各种锁
// Created by wangww on 2020/4/7.
#include <fcntl.h>
#include <iostream>

void show_lock_info(struct flock *);

int lock_file_area(){
    using std::cout;
    using std::cerr;
    const char *test_file = "tmp/test_lock";
    const int SIZE_TO_TRY = 5;

    // open a file desc
    int file_desc = open(test_file, O_RDWR | O_CREAT, 0666);
    if (!file_desc){
        std::cerr << "Unable to open " << std::string(test_file) << " for read/write";
        exit(EXIT_FAILURE);
    }

    // 设置希望测试的文件区域
    for (int start_byte = 0; start_byte < 99; start_byte += SIZE_TO_TRY) {
        struct flock region_to_test = {
                start_byte,
                SIZE_TO_TRY,
                -1,
                SEEK_SET,
                F_WRLCK
        };

        std::cout << " Testing F_WRLCK on region from " << start_byte << " to " << start_byte + SIZE_TO_TRY << std::endl;
        int res = fcntl(file_desc, F_GETLK, &region_to_test);
        if (res == -1){
            cerr << "F_GETLK failed\n";
            exit(EXIT_FAILURE);
        }
        if (region_to_test.l_pid != -1){
            cout << "Lock would fail, F_GETLK returned:\n";
            show_lock_info(&region_to_test);
        } else
            cout << "F_WRLCK - Lock would succeed\n";

        // 使用共享锁重复测试一次，再次设置希望测试的文件区域
        region_to_test.l_type = F_RDLCK;
        region_to_test.l_pid = -1;
        cout << "Testing F_RDLCK on region from " << start_byte << " to " << start_byte + SIZE_TO_TRY << std::endl;

        // 再次测试文件上的锁
        res = fcntl(file_desc, F_GETLK, &region_to_test);
        if (res == -1){
            cerr << "F_GETLK failed\n";
            exit(EXIT_FAILURE);
        }
        if (region_to_test.l_pid != -1){
            cout << "Lock would fail, F_GETLK returned:\n";
            show_lock_info(&region_to_test);
        } else
            cout << "F_RDLCK - Lock would succeed\n";
    }
    return 0;
}

void show_lock_info(struct flock *to_show){
    using std::cout;
    cout << "\tl_type " << to_show -> l_type;
    cout << "\tl_whence " << to_show -> l_whence;
    cout << "\tl_start " << to_show -> l_start;
    cout << "\tl_len " << to_show -> l_len;
    cout << "\tl_pid " << to_show -> l_pid << std::endl;
}

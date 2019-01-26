#递归获取所有当前目录及子目录下的C文件
#file(GLOB_RECURSE c_files ./*.c)

#生成测试执行文件
# 冒泡排序测试
add_executable(bubble_sort_test base/sort/bubble_sort_test.c)

#链接外部库
target_link_libraries(bubble_sort_test libacm-algrithm.a)
#递归获取所有当前目录及子目录下的C文件
file(GLOB_RECURSE c_files ./*.c)

#生成执行文件
add_executable(acm_test ${c_files})

#链接外部库
target_link_libraries(acm_test libacm-algrithm.a)
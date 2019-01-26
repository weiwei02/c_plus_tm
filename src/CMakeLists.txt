 #递归获取当前目录及子目录下的所有c文件
 file(GLOB_RECURSE c_files ./*.c)
# file(GLOB_RECURSE c_files "*.c")
#set(c_files base/sort/bubble_sort.c)
 #生成动态库和静态库
 add_library(acm_algrithm_lib_shared  SHARED ${c_files})
 add_library(acm_algrithm_lib_static STATIC ${c_files})

 #将动态库和静态库的名字设置为一致
 set_target_properties(acm_algrithm_lib_shared PROPERTIES OUTPUT_NAME "acm-algrithm")
 set_target_properties(acm_algrithm_lib_static PROPERTIES OUTPUT_NAME "acm-algrithm")

 #设置动态库版本
 set_target_properties(acm_algrithm_lib_shared PROPERTIES VERSION 1.2 SOVERSION 1)

 #安装动态库和静态库
 INSTALL(TARGETS acm_algrithm_lib_shared acm_algrithm_lib_static
         LIBRARY DESTINATION lib
         ARCHIVE DESTINATION lib)

 #安装头文件
 INSTALL(DIRECTORY ${PROJECT_SOURCE_DIR}/include/ DESTINATION include)
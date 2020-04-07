#递归获取当前目录及子目录下的所有c文件
#file(GLOB_RECURSE c_files ./*.c)
 file(GLOB_RECURSE c_files ./*.c*)
#set(c_files base/sort/bubble_sort.c)
#生成动态库和静态库
add_library(${PROJECT_NAME}_lib_shared  SHARED ${c_files})
add_library(${PROJECT_NAME}_lib_static STATIC ${c_files})

#将动态库和静态库的名字设置为一致
set_target_properties(${PROJECT_NAME}_lib_shared PROPERTIES OUTPUT_NAME "${PROJECT_NAME}")
set_target_properties(${PROJECT_NAME}_lib_static PROPERTIES OUTPUT_NAME "${PROJECT_NAME}")

#设置动态库版本
set_target_properties(${PROJECT_NAME}_lib_shared PROPERTIES VERSION 1.2 SOVERSION 1)

#设置 reentrant 宏定义，用于创建支持线程并发的函数
set(_REENTRANT)
#安装动态库和静态库
#INSTALL(TARGETS ${PROJECT_NAME}_lib_shared ${PROJECT_NAME}_lib_static
#        LIBRARY DESTINATION lib
#        ARCHIVE DESTINATION lib)

INSTALL(TARGETS ${PROJECT_NAME}_lib_static ${PROJECT_NAME}_lib_shared
        LIBRARY DESTINATION lib
        ARCHIVE DESTINATION lib)

#安装头文件
INSTALL(DIRECTORY include/mem DESTINATION include)
#INSTALL(DIRECTORY include/process DESTINATION include)
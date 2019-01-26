cmake_minimum_required (VERSION 2.8.4)
set(CMAKE_BUILD_TYPE Debug)
project (acm-algrithm)

#ADD_SUBDIRECTORY(src bin)

#SET(CMAKE_INSTALL_PREFIX ${PROJECT_BINARY_DIR})
#SET(CMAKE_INSTALL_PREFIX /usr/local)
#设置库文件存放路径
#设置安装目录
set(CMAKE_INSTALL_PREFIX ${PROJECT_SOURCE_DIR}/install)
set(LIBRARY_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/build/lib)

#设置执行文件存放路径
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/build/bin)

#获取当前目录及子目录(递归获取),添加到头文件搜索路径
function(include_sub_directories_recursively root_dir)
    if (IS_DIRECTORY ${root_dir})               # 当前路径是一个目录吗，是的话就加入到包含目录
        message("include dir: " ${root_dir})
        include_directories(${root_dir})
    endif()

    file(GLOB ALL_SUB RELATIVE ${root_dir} ${root_dir}/*) # 获得当前目录下的所有文件，让如ALL_SUB列表中
    foreach(sub ${ALL_SUB})
        if (IS_DIRECTORY ${root_dir}/${sub})
            include_sub_directories_recursively(${root_dir}/${sub}) # 对子目录递归调用，包含
        endif()
    endforeach()
endfunction()

#项目的所有目录都为头文件搜索路径
include_sub_directories_recursively(${PROJECT_SOURCE_DIR}/src)

#添加库文件搜索路径
link_directories(
        ${PROJECT_SOURCE_DIR}/build/lib
)

#添加子目录
add_subdirectory(src)
add_subdirectory(test)


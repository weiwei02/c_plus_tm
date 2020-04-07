
file(GLOB_RECURSE c_files2 ./*.c*)
add_library(${PROJECT_NAME}1_lib_shared  SHARED ${c_files2})
# 单元测试
add_executable(helloWorld hello/main.c)

add_executable(t_realloc mem/TestRealloca.c)
#链接外部库
target_link_libraries(t_realloc lib${PROJECT_NAME}.a)

add_executable(t_memset mem/TestMemset.c)
target_link_libraries(t_memset lib${PROJECT_NAME}.a)

add_executable(t_linklist mem/TestLinklist.c)
target_link_libraries(t_linklist lib${PROJECT_NAME}.a)

# fork 进程测试
add_executable(t_fork process/TestProcess.c)
target_link_libraries(t_fork lib${PROJECT_NAME}.a)
# fvork 进程测试
add_executable(t_vfork process/TestVFork.c)
target_link_libraries(t_vfork lib${PROJECT_NAME}.a)

# wait测试
add_executable(t_wait process/TestWait.c)
target_link_libraries(t_wait lib${PROJECT_NAME}.a)

# 创建线程测试
add_executable(t_createThread process/TestCreationThread.c)
target_link_libraries(t_createThread lib${PROJECT_NAME}.a)

# c++ 基本测试
add_executable(t_ptrstr cxx/TestPtrstr.cpp)
target_link_libraries(t_ptrstr lib${PROJECT_NAME}.a)

# c++ 基本测试
add_executable(t_stock cxx/TestStock.cc)
target_link_libraries(t_stock lib${PROJECT_NAME}.a)

# c++ 测试线程信号量同步
add_executable(t_thread_semaphore process/TestTheadSem.cc)
target_link_libraries(t_thread_semaphore lib${PROJECT_NAME}.a)

#c++ 测试线程detach
add_executable(t_thread_detach process/TestTheadDetach.cc)
target_link_libraries(t_thread_detach lib${PROJECT_NAME}.a)

#c++ 测试线程同步
add_executable(t_thread_mutex process/TestTheadMutex.cc)
target_link_libraries(t_thread_mutex lib${PROJECT_NAME}.a)
#c++ 测试标准库线程
add_executable(t_thread_std process/TestTheadStd.cc)
target_link_libraries(t_thread_std lib${PROJECT_NAME}.a)

#c++ 测试继承
add_executable(t_accabc cxx/acctacb.cc)
target_link_libraries(t_accabc lib${PROJECT_NAME}.a)

#c++ 测试继承2
add_executable(t_usedma cxx/usedma.cc)
target_link_libraries(t_usedma lib${PROJECT_NAME}.a)


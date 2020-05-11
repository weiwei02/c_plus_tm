package(default_visibility = ["//visibility:public"])

# https://docs.bazel.build/versions/master/be/c-cpp.html#cc_binary
cc_binary(
    name = "tcp_socket_test",
    srcs = ["TestTcpSocket.cc"],
    # copts = [],
    deps = [
        "//src:sockets-lib"
    ]
)

cc_binary(
    name = "select_stdin_test",
    srcs = ["TestSelectStdIn.cc"],
    # copts = [],
    deps = [
        "//src:sockets-lib"
    ]
)

# 测试poll函数
cc_binary(
    name = "poll_pipe_test",
    srcs = ["TestPollPipe.cc"],
    # copts = [],
    deps = [
        "//src:sockets-lib"
    ]
)

# 测试libevent函数
cc_binary(
    name = "simple_event_test",
    srcs = ["event_simple_test.cc"],
    # copts = [],
    deps = [
        "//src:sockets-lib"
    ]
)
# 测试libevent buffer 函数
cc_binary(
    name = "event_buffer_test",
    srcs = ["event_buffer_test.cc"],
    # copts = [],
    deps = [
        "//src:sockets-lib"
    ]
)
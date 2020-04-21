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
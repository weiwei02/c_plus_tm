package(default_visibility = ["//visibility:public"])
cc_library(
    name = "cxx-lib",
    srcs = glob(["cxx/source/*.cc",
                 "cxx/source/*.cpp",]),
    hdrs = glob(["cxx/include/*.h"]),
    includes = ["cxx"]
)

# 内核文件接口
cc_library(
    name = "files-lib",
    srcs = glob(["files/source/*.cc",
                 "files/source/*.c",]),
    hdrs = glob(["files/include/*.h"]),
    includes = ["files/include"]
)

# 内核进程管理接口
cc_library(
    name = "process-lib",
    srcs = glob(["process/*.cc",
                 "process/*.c",]),
    hdrs = glob(["process/include/*.h"]),
    includes = ["process/include"]
)

cc_library(
    name = "ipc-lib",
    srcs = glob(["ipc/source/*.cc",]),
    hdrs = glob(["ipc/include/*.h"]),
#    hdrs = ["cxx/include/AcctAbc.h",
#            "cxx/include/base.h",
#            "cxx/include/Dma.h",
#            "cxx/include/stock10.h",
#            ],
    includes = ["ipc"]
)

cc_library(
    name = "threadpool-lib",
#    srcs = glob(["ipc/source/*.cc",]),
    hdrs = [
        "threadpool/SyncQueue.h",
        "threadpool/ThreadPool.h"
            ],
    includes = ["threadpool"]
)

# socket 通信
cc_library(
    name = "sockets-lib",
    srcs = glob(["sockets/source/*.cc",]),
    hdrs = [
        "sockets/include/sockets.h",
        "sockets/include/libev.h",
        ],
    includes = ["sockets"],
    deps = [
        "//bazel:libevent"
    ]
)
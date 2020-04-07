package(default_visibility = ["//visibility:public"])
cc_library(
    name = "cxx-lib",
    srcs = glob(["cxx/source/*.cc",
                 "cxx/source/*.cpp",]),
    hdrs = glob(["cxx/include/*.h"]),
#    hdrs = ["cxx/include/AcctAbc.h",
#            "cxx/include/base.h",
#            "cxx/include/Dma.h",
#            "cxx/include/stock10.h",
#            "cxx/Include/Tv.h",
#            "cxx/Include/Stdmove.h"
#            ],
    includes = ["cxx"]
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
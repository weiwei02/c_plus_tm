cc_binary(
    name = "usedma",
    srcs = ["usedma.cc"],
    deps = [
            "//src:cxx-lib",
        ],
)

cc_binary(
    name = "acctacb",
    srcs = ["acctacb.cc"],
    deps = [
            "//src:cxx-lib",
        ],
)


#测试友元类
cc_test(
    name = "tv_test",
    srcs = ["tv_test.cc"],
#    copts = ["-Iexternal/gtest/include"],
    deps = [
        "@com_google_googletest//:gtest_main",
        "//src:cxx-lib",
    ],
)

#测试右值
cc_test(
    name = "move_test",
    srcs = ["TestMove.cc"],
#    copts = ["-Iexternal/gtest/include"],
    deps = [
        "@com_google_googletest//:gtest_main",
        "//src:cxx-lib",
    ],
)

#测试右值
cc_binary(
    name = "vector_test",
    srcs = ["TestVector.cc"],
#    copts = ["-Iexternal/gtest/include"],
    deps = [
        "//src:cxx-lib",
    ],
)

#测试线程池
cc_binary(
    name = "threadpool_test",
    srcs = ["TestThreadPool.cc"],
#    copts = ["-Iexternal/gtest/include"],
    deps = [
        "//src:threadpool-lib",
    ],
)
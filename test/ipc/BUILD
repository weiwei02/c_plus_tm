package(default_visibility = ["//visibility:public"])

cc_test(
    name = "pipe-test",
    srcs = ["TestPipe.cc"],
    copts = ["-Iexternal/gtest/include"],
    deps = [
#        "@gtest//:main",
        "@com_google_googletest//:gtest_main",
        "//src:ipc-lib"
    ],
)

cc_test(
    name = "sysv-sem-test",
    srcs = ["TestSysvSem.cc"],
    # copts = ["-Iexternal/gtest/include"],
    deps = [
#        "@gtest//:main",
        "@com_google_googletest//:gtest_main",
        "//src:ipc-lib"
    ],
)
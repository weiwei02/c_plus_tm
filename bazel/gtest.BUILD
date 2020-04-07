cc_library(
    name = "main",
    srcs = glob(
        ["src/*.cc"],
        exclude = ["src/gtest-all.cc"]
    ),
    hdrs = glob([
        "include/**/*.h",
        "src/*.h"
    ]),
    copts = [
        "-Iexternal/gtest/include",
        "-Iexternal/gtest/googletest-release-1.7.0"
    ],
    linkopts = ["-pthread"],
    visibility = ["//visibility:public"],
)
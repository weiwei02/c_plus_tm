workspace(name = "c_plus_tm")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "gtest",
    url = "https://github.com/google/googletest/archive/release-1.7.0.zip",
    sha256 = "b58cb7547a28b2c718d1e38aee18a3659c9e3ff52440297e965f5edffe34b6d0",
    build_file = "@//bazel:gtest.BUILD",
    strip_prefix = "googletest-release-1.7.0",
)

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")


git_repository(
    name = "com_google_googletest",
    remote = "https://github.com/google/googletest.git",
    tag = "release-1.10.0",
)
#new_http_archive(
#    name="asio_repository",
#    urls=["https://github.com/chriskohlhoff/asio/archive/asio-1-14-0.tar.gz"],
#    strip_prefix = "asio-1-14-0",
#    build_file = "asio.BUILD"
#)

#new_local_repository(
#    name = "asio",
#    path = "/Users/wangww/CLionProjects/external/asio/asio",
#    build_file = "asio.BUILD"
#)

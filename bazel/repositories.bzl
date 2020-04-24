load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

BUILD_ALL_CONTENT = """filegroup(name = "all", srcs = glob(["**"]), visibility = ["//visibility:public"])"""

def dependency_repositories():
    http_archive(
        name = "rules_foreign_cc",
        sha256 = "3184c244b32e65637a74213fc448964b687390eeeca42a36286f874c046bba15",
        strip_prefix = "rules_foreign_cc-7bc4be735b0560289f6b86ab6136ee25d20b65b7",
        # 2019-09-26
        urls = ["https://github.com/bazelbuild/rules_foreign_cc/archive/7bc4be735b0560289f6b86ab6136ee25d20b65b7.tar.gz"],
    )
    http_archive(
        name = "libevent",
        build_file_content = BUILD_ALL_CONTENT,
        strip_prefix = "libevent-2.1.8-stable",
        urls = [
            "https://github.com/libevent/libevent/releases/download/release-2.1.8-stable/libevent-2.1.8-stable.tar.gz",
        ],
    )
    http_archive(
        name = "com_github_libevent",
        sha256 = "549d34065eb2485dfad6c8de638caaa6616ed130eec36dd978f73b6bdd5af113",
        strip_prefix = "libevent-0d7d85c2083f7a4c9efe01c061486f332b576d28",
        urls = ["https://github.com/libevent/libevent/archive/0d7d85c2083f7a4c9efe01c061486f332b576d28.tar.gz"],
        build_file_content = BUILD_ALL_CONTENT,
        # patch_args = ["-p0"],
        # native.bind(
        #     name = "event",
        #     actual = "@envoy//bazel/foreign_cc:event",
        # )
        # build_file = "@//bazel:gtest.BUILD",
    )  

def test_respositories():
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

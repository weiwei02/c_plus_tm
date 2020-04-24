workspace(name = "c_plus_tm")

load("//bazel:repositories.bzl", "dependency_repositories")
dependency_repositories()


load("@rules_foreign_cc//:workspace_definitions.bzl", "rules_foreign_cc_dependencies")
rules_foreign_cc_dependencies([
    # "//:built_cmake_toolchain",
    # "//:built_ninja_toolchain_osx",
    # "//:built_ninja_toolchain_linux",
])

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")
bazel_skylib_workspace()

load("//bazel:repositories.bzl", "test_respositories")
test_respositories()



# External CMake C++ library targets should be specified with this function. This defaults
# to building the dependencies with ninja
def envoy_cmake_external(
        name,
        cache_entries = {},
        debug_cache_entries = {},
        cmake_options = ["-GNinja"],
        make_commands = ["ninja -v", "ninja -v install"],
        lib_source = "",
        postfix_script = "",
        static_libraries = [],
        copy_pdb = False,
        pdb_name = "",
        cmake_files_dir = "$BUILD_TMPDIR/CMakeFiles",
        generate_crosstool_file = False,
        **kwargs):
    cache_entries.update({"CMAKE_BUILD_TYPE": "Bazel"})
    cache_entries_debug = dict(cache_entries)
    cache_entries_debug.update(debug_cache_entries)

    pf = ""
    if copy_pdb:
        # TODO: Add iterator of the first list presented of these options;
        # static_libraries[.pdb], pdb_names, name[.pdb] files
        if pdb_name == "":
            pdb_name = name

        copy_command = "cp {cmake_files_dir}/{pdb_name}.dir/{pdb_name}.pdb $INSTALLDIR/lib/{pdb_name}.pdb".format(cmake_files_dir = cmake_files_dir, pdb_name = pdb_name)
        if postfix_script != "":
            copy_command = copy_command + " && " + postfix_script

        pf = select({
            "@envoy//bazel:windows_dbg_build": copy_command,
            "//conditions:default": postfix_script,
        })
    else:
        pf = postfix_script

    cmake_external(
        name = name,
        cache_entries = select({
            "@envoy//bazel:opt_build": cache_entries,
            "//conditions:default": cache_entries_debug,
        }),
        cmake_options = cmake_options,
        # TODO(lizan): Make this always true
        generate_crosstool_file = select({
            "@envoy//bazel:windows_x86_64": True,
            "//conditions:default": generate_crosstool_file,
        }),
        lib_source = lib_source,
        make_commands = make_commands,
        postfix_script = pf,
        static_libraries = static_libraries,
        **kwargs
    )
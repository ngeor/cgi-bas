load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Rust rules

http_archive(
    name = "rules_rust",
    sha256 = "e968b3d01c305282bd237bcdafe5e5141192e93eaefb22712e8bc4299e672b16",
    strip_prefix = "rules_rust-262b6a5ea17ec91b3c07e37e82af0eb15dd6ceef",
    urls = [
        # `main` branch as of 2021-08-23
        "https://github.com/bazelbuild/rules_rust/archive/262b6a5ea17ec91b3c07e37e82af0eb15dd6ceef.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

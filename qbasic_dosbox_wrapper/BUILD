load("@rules_rust//rust:defs.bzl", "rust_binary")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "qbasic_dosbox_wrapper",
    srcs = ["main.rs"],
    deps = [
        "//dosbox_lib",
    ],
)

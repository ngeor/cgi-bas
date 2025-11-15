## [unreleased]

### 🚀 Features

- Added qbasic_cgi_bin script
- Implement rusty_basic cgi-bin script
- [**breaking**] Removed basic-launcher-rust and perf

### 🚜 Refactor

- Move dosbox_lib to its own package
- Added basic_lib

### 🎨 Styling

- Styling changelog according to default options

### ⚙️ Miscellaneous Tasks

- Updated copyright year in LICENSE
- Adding .github\FUNDING.yml
- *(changelog)* Updated changelog
- Use cliff.toml from instarepo
- *(changelog)* Updated changelog
- Added github actions workflow
- Removed funding file
- Removed .gitattributes
## [0.1.0] - 2021-12-08

### 🚀 Features

- Removed rand dependency from basic-launcher-rust
- Introducing Bazel
- Implementing new wrappers
- Cgi_test script to troubleshoot Apache
- *(dosbox_wrapper)* Support pass-through environment variables
- Adding gwbasic wrapper to used as cgi-bin handler
- Support for running gw-basic through cgi-bin
- Introducing git-cliff
- Add find_dosbox method in DOSBox struct

### 🐛 Bug Fixes

- Fixed warning on panic not using formatting

### 🚜 Refactor

- Moved Bazel rules in basic/rules
- Use the builder pattern for dosbox_wrapper
- Flatten dosbox_lib exports

### ⚙️ Miscellaneous Tasks

- Moved files one level up

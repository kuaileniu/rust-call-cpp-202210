# rust-cpp-202210
Rust项目中含C++源代码

# 可在cpp源码中打断点进行调试
- IDE中 启动debug

# 初次编译
CARGO_NET_GIT_FETCH_WITH_CLI=true  cargo run
# package release
CARGO_NET_GIT_FETCH_WITH_CLI=true  cargo build --release

# cc 库
- 支持 c 、 cpp 与 Rust混合编程
- 支持 CUDA
- https://github.com/rust-lang/cc-rs

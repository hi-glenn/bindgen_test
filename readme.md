#### 文章链接
这是一个链接 [菜鸟教程](https://rustcc.cn/article?id=9219a366-84d3-49c8-b957-dfbade1257fc)

#### 安装依赖
```bash
apt install libsecp256k1-dev
```

#### 绑定
```bash
cargo build

# 生成的绑定文件位于OUT_DIR/bindings.rs
# 其中$OUT_DIR由 cargo 根据 build.rs 确定，
# 默认类似于./target/debug/build/crate-package-name-afc7747d7eafd720/out/
```

#### 运行 lib.rs 中的实例
```bash
cargo test --package playground --lib -- test_create_pubkey --exact --nocapture
```

#### cbingen 与 bindgen
```bash
Cbindgen 是一个类似于 bindgen 的工具，但它为 Rust 代码生成 C/C++ FFI 绑定，而不是为 C/C++ 代码生成 Rust 绑定。
Cbindgen 将 Rust 库作为输入，并生成可用于从 C/C++ 代码调用 Rust 函数和类型的 C/C++ 头文件。
与 bindgen 一样，cbindgen 使用 clang 来解析 Rust 代码并提取有关其中定义的类型和函数的信息。
```
![这是图片](/bindgen.png "Magic Gardens")
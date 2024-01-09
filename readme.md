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

一个带有获取构建目录的XTASK模版


xtask介绍：
https://github.com/matklad/cargo-xtask

issues讨论：
https://github.com/rust-lang/rfcs/pull/1777
https://github.com/rust-lang/cargo/issues/3815



当有构建后需求时，代码写在xtask/src/main.rs中

执行：cargo xtask --release --target=aarch64-apple-darwin
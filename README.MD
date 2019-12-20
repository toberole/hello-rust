Cargo.toml 为 Rust 的清单文件。其中包含了项目的元数据和依赖库

Rust中变量默认是不可以变的，可以使用mut声明变量可变

运行 cargo doc --open 命令来构建所有本地依赖提供的文档，并在浏览器中打开。
rand crate 中的其他功能感兴趣，可以运行 cargo doc --open 并点击左侧导航栏中的 rand。

一个包可以包含多个二进制 crate 项和一个可选的 crate 库
模块系统（the module system）包括：
    包（Packages）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
    Crates ：一个模块的树形结构，它形成了库或二进制项目。
    模块（Modules）和 use： 允许你控制作用域和路径的私有性。
    路径（path）：一个命名例如结构体、函数或模块等项的方式
包（package） 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件，阐述如何去构建这些 crate。

包中所包含的内容由几条规则来确立。一个包中至多 只能 包含一个库 crate(library crate)；包中可以包含任意多个二进制 crate(binary crate)；包中至少包含一个 crate，无论是库的还是二进制的。

路径有两种形式：
绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。





















































































































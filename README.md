# complier
应该是算是lox的方言
P语言，请原谅这门语言取名如此简陋，毕竟我菜。
## 中文翻译
https://readonly.link/books/https://raw.githubusercontent.com/GuoYaxiang/craftinginterpreters_zh/main/book.json
## 源代码仓库

## 阿b教程
https://www.bilibili.com/video/BV1Ax4y1t7T1/?spm_id_from=333.1007.top_right_bar_window_default_collection.content.click
## 下一步修改计划
1. 添加自增代码 += -= *= /=
2. 修正代码解析的问题 已完成
3. 增加struct 优先级低 已完成
4. 是否区分静态函数和非静态函数
### 编译rust到wasm
`cargo build --target wasm32-unknown-unknown --release`
### 生成对应的js代码
`wasm-bindgen --out-dir ./out --target web target/wasm32-unknown-unknown/release/p.wasm`
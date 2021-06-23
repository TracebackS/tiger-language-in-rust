# tiger-language-in-rust

使用`rust`语言实现《现代编译原理》中定义的`tiger`语言编译器，并按照原实验格式，按分支组织成各独立实验。

## AST

使用[pest](https://pest.rs)库作为解析器生成器，使用[PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar)描述文法，生成AST。

分支`ast-dev`:

- 在`src/pest-src/tiger.pest`中编写文法

- 运行`cargo run ./testcases/<filename>.tig`，查看解析结果

## WIP

后续内容待开发。

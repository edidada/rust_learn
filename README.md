# README

```shell
3 |     let x: i32;
|         - binding declared here but left uninitialized
4 |
5 |     println!("Number {x}");
|                       ^ `x` used here but it isn't initialized
```


```
error[E0384]: cannot assign twice to immutable variable `x`
--> src\exercises\01_variables\variables4.rs:6:5
|
3 |     let  x = 3;
|          - first assignment to `x`
...
6 |     x = 5; // Don't change this line
|     ^^^^^ cannot assign twice to immutable variable
|
```

## 2015

## 2018

## 2021
rust 2018     let numbers = [27, 297, 38502, 81];
let division_results = numbers.into_iter().map(|n| divide(*n, 27));
2021     let numbers = [27, 297, 38502, 81];
let division_results = numbers.into_iter().map(|n| divide(n, 27));
为啥？区别

您观察到的现象确实存在，这与 Rust 2021 edition 中的一个重要变更有关：闭包捕获规则的改变。

关键差异

2018 Edition

let numbers = [27, 297, 38502, 81];
let division_results = numbers.into_iter().map(|n| divide(*n, 27));
// 需要 *n 解引用


2021 Edition

let numbers = [27, 297, 38502, 81];
let division_results = numbers.into_iter().map(|n| divide(n, 27));
// 不需要 *n，自动解引用


原因：Rust 2021 的闭包捕获改进

在 Rust 2021 中，闭包捕获规则进行了优化，现在能自动解引用某些类型。具体来说：

2018 版本的问题

// 2018: 这会产生编译错误或需要显式解引用
numbers.into_iter().map(|n| divide(n, 27));
// 错误：n 是 &i32，但 divide 期望 i32


2021 版本的改进

// 2021: 自动解引用
numbers.into_iter().map(|n| divide(n, 27));
// 正确：编译器会自动从 &i32 解引用为 i32


实际例子

fn divide(a: i32, b: i32) -> i32 {
a / b
}

fn main() {
let numbers = [27, 297, 38502, 81];

    // 2018 写法（需要 *n）
    let results_2018: Vec<_> = numbers
        .into_iter()
        .map(|n| divide(*n, 27))  // 必须解引用
        .collect();
    
    // 2021 写法（自动解引用）
    let results_2021: Vec<_> = numbers
        .into_iter()
        .map(|n| divide(n, 27))  // 自动解引用
        .collect();
    
    println!("Results: {:?}", results_2021);
}


背后的原理

1. 类型匹配：当闭包参数类型与函数期望类型不完全匹配时
2. 自动转换：Rust 2021 增加了更多的自动解引用和类型转换
3. Deref 特质：对实现了 Deref trait 的类型自动解引用

其他例子

// 字符串切片也有类似改进
let words = ["hello", "world"];

// 2018
let lengths_2018: Vec<_> = words
.iter()
.map(|s| s.len())  // s 是 &&str，但 .len() 可用
.collect();

// 2021 - 更灵活
let lengths_2021: Vec<_> = words
.iter()
.map(|s| s.to_uppercase())  // 自动解引用多次
.collect();


重要注意事项

1. 不是所有情况都自动解引用：
   // 这仍然需要手动处理
   let nums = &[1, 2, 3];
   let sum: i32 = nums.iter().sum();  // 正确
   // nums.iter().map(|n| n + 1) 可能需要 *n


2. into_iter() vs iter()：
   // into_iter() 产生值
   [1, 2, 3].into_iter().map(|n| n + 1);  // n: i32

   // iter() 产生引用
   [1, 2, 3].iter().map(|n| n + 1);  // 2021: 自动解引用

迁移建议
从 2018 升级到 2021 时，可以：
1. 删除不必要的 * 操作符
2. 让编译器提示是否需要解引用
3. 使用 cargo fix --edition 自动修复
这个改进让 Rust 代码更加简洁，减少了样板代码。

## 2024

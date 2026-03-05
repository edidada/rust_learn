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


## 2024

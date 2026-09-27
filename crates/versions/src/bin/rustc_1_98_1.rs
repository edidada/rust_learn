// rustc 1.98.1 演示 —— patch：vtable 生成 miscompilation 修复（记叙，无新 API）
fn main() {
    println!("rustc 1.98.1 演示（patch 版）");

    println!("\n1. vtable 生成错误编译修复");
    println!("vtable = trait 对象的虚表（方法指针 + 布局信息）；1.98.0 生成它时有误，本 patch 修复");
    // 简单 trait 对象回归自检（无新 API）
    trait Speak {
        fn speak(&self) -> String;
    }
    struct Dog;
    impl Speak for Dog {
        fn speak(&self) -> String {
            "wang".into()
        }
    }
    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog)];
    println!("dyn Trait 虚调用回归：{}", animals[0].speak());
}

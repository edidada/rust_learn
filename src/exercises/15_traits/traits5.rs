trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: Fix the compiler error by only changing the signature of this function.
// 使用 `impl Trait` 语法指定参数需要同时实现两个 trait
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
    let some_struct = SomeStruct;
    let other_struct = OtherStruct;
    
    println!("some_func(SomeStruct) = {}", some_func(some_struct));
    println!("some_func(OtherStruct) = {}", some_func(other_struct));
    
    // 演示更多用法
    let result1 = some_func(SomeStruct);
    let result2 = some_func(OtherStruct);
    println!("\n测试结果:");
    println!("SomeStruct: {}", result1);
    println!("OtherStruct: {}", result2);
    
    // 创建一个只实现 SomeTrait 的结构体
    struct OnlySomeStruct;
    impl SomeTrait for OnlySomeStruct {}
    
    // 这行会编译错误，因为 OnlySomeStruct 没有实现 OtherTrait
    // let result3 = some_func(OnlySomeStruct);  // ❌
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
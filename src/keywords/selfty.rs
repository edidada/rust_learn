// SelfTy 关键字示例

// 在trait中使用Self
trait MyTrait {
    fn create_self() -> Self;
    fn get_self(&self) -> &Self;
}

// 在impl块中使用Self
struct MyStruct {
    value: i32,
}

impl MyTrait for MyStruct {
    // Self 指代当前实现的类型 MyStruct
    fn create_self() -> Self {
        Self { value: 42 }
    }
    
    fn get_self(&self) -> &Self {
        self
    }
}

fn main() {
    let instance = MyStruct::create_self();
    println!("Created instance with value: {}", instance.value);
    
    let self_ref = instance.get_self();
    println!("Self reference value: {}", self_ref.value);
}
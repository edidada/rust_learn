// any 模块示例：动态类型和类型反射
use std::any::{Any, TypeId};

fn main() {
    // 创建不同类型的值
    let value_i32: i32 = 42;
    let value_string: String = "Hello".to_string();
    let value_bool: bool = true;
    
    // 将值转换为Box<dyn Any>
    let any_i32: Box<dyn Any> = Box::new(value_i32);
    let any_string: Box<dyn Any> = Box::new(value_string);
    let any_bool: Box<dyn Any> = Box::new(value_bool);
    
    // 检查类型并提取值
    if let Some(v) = any_i32.downcast_ref::<i32>() {
        println!("i32 value: {}", v);
    }
    
    if let Some(v) = any_string.downcast_ref::<String>() {
        println!("String value: {}", v);
    }
    
    if let Some(v) = any_bool.downcast_ref::<bool>() {
        println!("bool value: {}", v);
    }
    
    // 获取类型ID
    let type_id_i32 = TypeId::of::<i32>();
    let type_id_string = TypeId::of::<String>();
    println!("Type ID of i32: {:?}", type_id_i32);
    println!("Type ID of String: {:?}", type_id_string);
}
// hash 模块示例：通用哈希支持
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// 1. 为自定义类型实现Hash trait
#[derive(Debug, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}

// 2. 自定义哈希实现
struct CustomHasher {
    state: u64,
}

impl Hasher for CustomHasher {
    fn finish(&self) -> u64 {
        self.state
    }
    
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state = self.state.wrapping_mul(31).wrapping_add(*byte as u64);
        }
    }
}

fn main() {
    // 1. 使用HashMap（内部使用哈希）
    println!("1. Using HashMap:");
    let mut map = HashMap::new();
    
    let person1 = Person { name: "Alice".to_string(), age: 30 };
    let person2 = Person { name: "Bob".to_string(), age: 25 };
    
    map.insert(person1, "Engineer");
    map.insert(person2, "Doctor");
    
    // 查找
    let search_person = Person { name: "Alice".to_string(), age: 30 };
    if let Some(job) = map.get(&search_person) {
        println!("Alice's job: {}", job);
    }
    
    // 2. 使用自定义哈希器
    println!("\n2. Using custom hasher:");
    let mut hasher = CustomHasher { state: 0 };
    "Hello".hash(&mut hasher);
    println!("Hash of 'Hello': {}", hasher.finish());
    
    // 3. 使用标准哈希器
    println!("\n3. Using standard hasher:");
    use std::collections::hash_map::DefaultHasher;
    
    let mut hasher2 = DefaultHasher::new();
    "Hello".hash(&mut hasher2);
    println!("Hash of 'Hello' (DefaultHasher): {}", hasher2.finish());
}
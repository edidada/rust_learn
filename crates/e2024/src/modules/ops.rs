// ops 模块示例：可重载运算符
use std::ops;

// 1. 实现加法运算符
struct Vector {
    x: i32,
    y: i32,
}

impl ops::Add for Vector {
    type Output = Vector;
    
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 2. 实现乘法运算符
impl ops::Mul<i32> for Vector {
    type Output = Vector;
    
    fn mul(self, scalar: i32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// 3. 实现索引运算符
struct MyArray {
    data: Vec<i32>,
}

impl ops::Index<usize> for MyArray {
    type Output = i32;
    
    fn index(&self, index: usize) -> &i32 {
        &self.data[index]
    }
}

impl ops::IndexMut<usize> for MyArray {
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        &mut self.data[index]
    }
}

fn main() {
    // 1. 使用加法运算符
    println!("1. Using addition operator:");
    let v1 = Vector { x: 1, y: 2 };
    let v2 = Vector { x: 3, y: 4 };
    let v3 = v1 + v2;
    println!("v1 + v2 = Vector {{ x: {}, y: {} }}", v3.x, v3.y);
    
    // 2. 使用乘法运算符
    println!("\n2. Using multiplication operator:");
    let v4 = Vector { x: 1, y: 2 };
    let v5 = v4 * 3;
    println!("v4 * 3 = Vector {{ x: {}, y: {} }}", v5.x, v5.y);
    
    // 3. 使用索引运算符
    println!("\n3. Using index operator:");
    let mut array = MyArray { data: vec![1, 2, 3, 4, 5] };
    println!("array[0] = {}", array[0]);
    println!("array[2] = {}", array[2]);
    
    // 修改值
    array[1] = 10;
    println!("After modification, array[1] = {}", array[1]);
}
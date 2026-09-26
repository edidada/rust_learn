// move 关键字示例

fn main() {
    // 1. 基本的move闭包
    let x = 5;
    let move_closure = move || {
        println!("x in closure: {}", x);
    };
    
    // 调用闭包
    move_closure();
    
    // 注意：x 仍然可以在外部使用，因为i32是Copy类型
    println!("x outside closure: {}", x);
    
    // 2. 对于非Copy类型
    let s = String::from("Hello");
    let move_closure_string = move || {
        println!("s in closure: {}", s);
    };
    
    // 调用闭包
    move_closure_string();
    
    // 注意：s 已经被移动到闭包中，不能在外部使用
    // println!("s outside closure: {}", s); // 这行会导致编译错误
    
    // 3. 作为函数参数
    fn takes_closure<F>(f: F) where F: Fn() {
        f();
    }
    
    let y = 10;
    takes_closure(move || {
        println!("y in closure: {}", y);
    });
}
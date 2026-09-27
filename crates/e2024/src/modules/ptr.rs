// ptr 模块示例：通过原始指针手动管理内存
use std::ptr;

fn main() {
    // 1. 原始指针创建
    println!("1. Creating raw pointers:");
    let mut x = 42;
    let ptr_x: *mut i32 = &mut x;
    let const_ptr_x: *const i32 = &x;
    
    println!("ptr_x: {:?}", ptr_x);
    println!("const_ptr_x: {:?}", const_ptr_x);
    
    // 2. 解引用原始指针
    println!("\n2. Dereferencing raw pointers:");
    unsafe {
        println!("*ptr_x: {}", *ptr_x);
        println!("*const_ptr_x: {}", *const_ptr_x);
        
        // 修改值
        *ptr_x = 100;
        println!("After modification, *ptr_x: {}", *ptr_x);
    }
    
    // 3. 空指针
    println!("\n3. Null pointers:");
    let null_ptr: *mut i32 = ptr::null_mut();
    let const_null_ptr: *const i32 = ptr::null();
    
    println!("null_ptr is null: {}", null_ptr.is_null());
    println!("const_null_ptr is null: {}", const_null_ptr.is_null());
    
    // 4. 指针算术
    println!("\n4. Pointer arithmetic:");
    let mut arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_mut_ptr();
    
    unsafe {
        println!("*ptr: {}", *ptr);
        println!("*(ptr + 1): {}", *(ptr + 1));
        println!("*(ptr + 2): {}", *(ptr + 2));
    }
    
    // 5. 指针比较
    println!("\n5. Pointer comparison:");
    let ptr1 = &x as *const i32;
    let ptr2 = &x as *const i32;
    let ptr3 = &arr[0] as *const i32;
    
    println!("ptr1 == ptr2: {}", ptr1 == ptr2);
    println!("ptr1 == ptr3: {}", ptr1 == ptr3);
    
    // 6. 内存复制
    println!("\n6. Memory copy:");
    let mut dest = [0; 5];
    let src = [1, 2, 3, 4, 5];
    
    unsafe {
        ptr::copy(src.as_ptr(), dest.as_mut_ptr(), 5);
        println!("Dest after copy: {:?}", dest);
    }
}
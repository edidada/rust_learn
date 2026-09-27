// rustc 1.15.1 演示 —— IntoIter::as_mut_slice 签名修复（补丁版）
fn main() {
    println!("rustc 1.15.1 演示（补丁：修复 IntoIter::as_mut_slice 签名）");

    println!("\n1. vec::IntoIter::as_slice / as_mut_slice");
    // 当年形态：1.15.0 中 as_mut_slice 签名有误；1.15.1 修复后可正常取别名视图
    let v = vec![1, 2, 3];
    let mut it = v.into_iter();
    println!("未消费时 as_slice = {:?}", it.as_slice());
    it.as_mut_slice()[0] = 100; // 原地改还没被消费的元素
    println!("as_mut_slice 改首元素后 = {:?}", it.as_slice());
    let first = it.next();
    println!("next() 取出 {:?}，剩余 as_slice = {:?}", first, it.as_slice());

    println!("\n2. builtins -fPIC 修复（println 讲解节）");
    // 当年形态：32 位平台上 compiler builtins 以 -fPIC 编译，
    // 解决共享对象链接场景下的重定位错误；属编译器构建层修复。
    println!("32 位平台 builtins 以 -fPIC 编译，便于链接进共享对象。");
}

// rustc 1.93.0 演示 —— MaybeUninit 切片写入/assume 系列、Vec::into_raw_parts、[T]::as_array、pop_*_if、fmt::from_fn
// 注意：本仓库以 rustc 1.98 运行；1.93 稳定的 API 在 1.98 均可用。
use std::fmt;
use std::mem::MaybeUninit;

fn main() {
    println!("rustc 1.93.0 演示");

    println!("\n1. <[MaybeUninit<T>]>::write_copy_of_slice / write_clone_of_slice（1.93 稳定）");
    // 旧写法：循环逐元素 ptr::write；1.93 一把梭整段写入
    let mut buf: [MaybeUninit<u8>; 8] = [MaybeUninit::uninit(); 8];
    let src: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    buf.write_copy_of_slice(&src); // 拷贝整段（Copy 类型走 memcopy）
    println!("write_copy_of_slice 后 buf = {:?}", unsafe { buf.assume_init_ref() });
    let src2: [String; 1] = [String::from("clone-me")];
    let mut buf2: [MaybeUninit<String>; 1] = [MaybeUninit::uninit(); 1];
    buf2.write_clone_of_slice(&src2); // 克隆整段（Clone 类型走 clone）
    println!("write_clone_of_slice 后 buf2 = {:?}", unsafe { buf2.assume_init_ref() });

    println!("\n2. assume_init_mut / assume_init_drop —— 记叙");
    println!("assume_init_mut:  &mut 视图，可就地初始化后读出");
    println!("assume_init_drop: 就地 drop 已初始化部分（Vec 内部这类缓冲用）");
    let mut m: [MaybeUninit<u8>; 2] = [MaybeUninit::uninit(); 2];
    let w = unsafe { m.assume_init_mut() }; // unsafe 调用；契约：缓冲必须已初始化
    w[0] = 10;
    w[1] = 20;
    println!("assume_init_mut 写 [10,20] 后 = {:?}", unsafe { m.assume_init_ref() });

    println!("\n3. Vec::into_raw_parts / String::into_raw_parts（1.93 稳定）");
    // (ptr, len, cap)；破坏性解构 —— 旧代码要 unsafe 拼 as_mut_ptr/len/capacity 三件套
    let v = vec![1i32, 2, 3];
    let (ptr, len, cap) = v.into_raw_parts();
    println!("into_raw_parts → ptr={:p} len={} cap={}", ptr, len, cap);
    let v2 = unsafe { Vec::from_raw_parts(ptr, len, cap) };
    println!("重组回去 v2 = {v2:?}");
    let (sptr, slen, scap) = String::from("abc").into_raw_parts();
    println!("String::into_raw_parts → len={} cap={}", slen, scap);
    drop(unsafe { String::from_raw_parts(sptr, slen, scap) });

    println!("\n4. <[T]>::as_array（数组化视图，1.93 稳定）");
    let arr4: Option<&[u8; 4]> = [0u8, 1, 2, 3, 4].as_array::<4>();
    println!("&[0..4] as_array::<4>() = {arr4:?}（None 若长度不足）");

    println!("\n5. VecDeque::pop_front_if / pop_back_if（1.93 稳定）");
    use std::collections::VecDeque;
    let mut q: VecDeque<i32> = [1, 2, 3].into();
    println!("pop_front_if(|x| *x < 2) = {:?}", q.pop_front_if(|x| *x < 2)); // Some(1)
    println!("pop_back_if(|x| *x > 5)  = {:?}", q.pop_back_if(|x| *x > 5)); // None
    println!("q 剩余 = {q:?}");

    println!("\n6. fmt::from_fn（1.93 进 const；本工具链稳定可用：闭包当 fmt 适配器）");
    // 旧写法：为一次性格式化写整个 Display/Debug impl；from_fn 一行搞定
    let shown = fmt::from_fn(|f| write!(f, "{:#010x}", 1234));
    println!("fmt::from_fn 格式化 1234 → {shown}");

    println!("\n7. Duration::from_nanos_u128 与 char::MAX_LEN_UTF8/UTF16");
    // u128 纳秒可以表达大跨度时长，不再受 u64 上限限制（超出可表达范围则 panic）
    let big: u128 = u64::MAX as u128; // 超过秒级上限、合法可表达
    println!("Duration::from_nanos_u128(u64::MAX as u128) = {:?}", std::time::Duration::from_nanos_u128(big));
    println!("char::MAX_LEN_UTF8  = {}", char::MAX_LEN_UTF8); // 4
    println!("char::MAX_LEN_UTF16 = {}", char::MAX_LEN_UTF16); // 2
}

// rustc 1.86.0 演示 —— trait 上转型 supertrait、#[target_feature] 安全函数、pop_if、get_disjoint_mut
// 注意：本仓库以 rustc 1.98 / edition 2024 运行；1.86 稳定的 API 在 1.98 均可用。

// 1.86 重点：trait 对象向 supertrait 上转型（trait object upcasting）。
// 旧行为：&dyn Dog 不能直接当 &dyn Animal 用（要手写 AsAnimal 转发 trait）；
// 1.86 起：Sub: Super 的 dyn 子对象可以直接上转型。
trait Animal {
    fn name(&self) -> String;
}
trait Dog: Animal {
    fn bark(&self) -> String;
}

struct Pug;
impl Animal for Pug {
    fn name(&self) -> String {
        "pug".into()
    }
}
impl Dog for Pug {
    fn bark(&self) -> String {
        "woof".into()
    }
}

// 接收 supertrait 对象的函数——1.86 前无法把 Box<dyn Dog> 直接传进来：
fn animal_speak(a: &dyn Animal) {
    println!("  animal_speak: {}", a.name());
}

// 1.86 稳定：安全函数标注 #[target_feature]
// 旧规则：target_feature 函数必须是 unsafe fn；1.86 起安全函数也可标注，
// 调用方需要具备相应特性（本机 x86_64 均有 sse2）。
#[target_feature(enable = "sse2")]
fn sse2_add(a: f64, b: f64) -> f64 {
    a + b
}

fn main() {
    println!("rustc 1.86.0 演示");

    println!("\n1. trait 对象 supertrait 上转型（1.86 重点）");
    let d: Box<dyn Dog> = Box::new(Pug);
    // ← 1.86 起直接上转型：Box<dyn Dog> → Box<dyn Animal>
    let a: Box<dyn Animal> = d;
    animal_speak(&*a);
    println!("bark 只能在 dyn Dog 上调（上转型会丢失子 trait 方法）");
    // 多级上转型也行：Dog: Animal，Animal 是 Super；引用同样可转：
    let pug = Pug;
    let sub: &dyn Dog = &pug;
    println!("直接调用 bark = {}", sub.bark()); // dyn Dog 上仍可调用子 trait 方法
    let sup: &dyn Animal = sub; // ← &dyn Dog → &dyn Animal（1.86 稳定）
    println!("&dyn Dog → &dyn Animal：name = {}", sup.name());

    println!("\n2. #[target_feature] 作用于安全函数（1.86 稳定）");
    // 1.86 稳定：安全函数标注 #[target_feature] —— 函数体是安全代码；
    // 但调用它仍要求"调用上下文具备该特性"（用 unsafe 块声明承诺，1.86 语义）：
    println!("sse2_add(1.5, 2.25) = {}", unsafe { sse2_add(1.5, 2.25) });

    println!("\n3. Vec::pop_if（1.86 稳定）");
    let mut v = vec![1, 2, 3, 4];
    // 条件为真才 pop；pop_if 接收 &mut T，还能"看一眼"再决定：
    let popped = v.pop_if(|x| *x > 3);
    println!("pop_if(>3) = {popped:?}, 剩余 = {v:?}");
    let popped2 = v.pop_if(|x| *x > 3);
    println!("pop_if(>3) 再试 = {popped2:?}, 剩余 = {v:?}");

    println!("\n4. HashMap::get_disjoint_mut（1.86 稳定）");
    let mut m = std::collections::HashMap::from([("a", 1), ("b", 2)]);
    // 一次可变借用多个 key（旧写法要两次 get_mut，借用检查器还常常不配合）。
    // 返回 [Option<&mut V>; N] 数组：
    let [a, b] = m.get_disjoint_mut(["a", "b"]);
    if let (Some(a), Some(b)) = (a, b) {
        *a += 10;
        *b += 20;
    }
    println!("map = {m:?}");

    println!("\n5. OnceLock::wait（1.86 稳定）");
    let cell = std::sync::OnceLock::new();
    cell.set(99).unwrap();
    // wait()：阻塞直到已初始化并返回值（已初始化则立即返回 Some）：
    println!("OnceLock::wait() = {:?}", cell.wait());

    println!("\n6. {{float}}::next_down / next_up（1.86 稳定）");
    let x = 1.0f64;
    println!("next_down(1.0) = {:?}, next_up(1.0) = {:?}", x.next_down(), x.next_up());
}
